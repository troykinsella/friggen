use std::borrow::Cow;
use std::collections::HashMap;
use std::io::{BufWriter, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{fs, io};

use tempfile::NamedTempFile;

use crate::error::Result;

const SCRIPT_DIR: &str = "~/.cache/friggen";
const SCRIPT_DIR_MODE: u32 = 0o750;

pub fn eval_shell_command(
    shell: &str,
    cmd: &str,
    env_vars: &HashMap<&str, &str>,
) -> Result<String> {
    let out = Command::new("/usr/bin/env")
        .arg(shell)
        .arg("-c")
        .arg(cmd)
        .envs(env_vars.iter())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if out.status.success() {
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        Ok(stdout)
    } else {
        io::stderr().write_all(&out.stderr)?;
        Err(anyhow::Error::msg(format!(
            "command failed ({}): {}",
            out.status.code().unwrap(),
            cmd
        ))
        .into())
    }
}

pub fn run_shell_script(
    hash_bang: &[&str],
    lines: &[&str],
    env_vars: &HashMap<&str, &str>,
    other_vars: &HashMap<&str, Cow<'_, str>>,
) -> Result<i32> {
    let script_dir = PathBuf::from(shellexpand::tilde(SCRIPT_DIR).to_string());
    if let Ok(dir_meta) = script_dir.metadata() {
        dir_meta.permissions().set_mode(SCRIPT_DIR_MODE);
    } else {
        fs::create_dir_all(&script_dir)?;
        script_dir
            .metadata()?
            .permissions()
            .set_mode(SCRIPT_DIR_MODE);
    }

    let script_file = &NamedTempFile::new_in(script_dir)?;
    let script_path = script_file.path();

    let size_guess = (lines.len() * 128).max(2048);
    let mut writer = BufWriter::with_capacity(size_guess, script_file);
    for line in lines {
        writer.write_all(line.as_bytes())?;
    }
    writer.flush()?;

    let mut hash_bang_components = hash_bang.iter();
    let mut child = Command::new(hash_bang_components.next().unwrap());
    for arg in hash_bang_components {
        child.arg(arg);
    }

    let code = child
        .envs(env_vars.iter())
        .envs(other_vars.iter().map(|e| (e.0, e.1.as_ref())))
        .arg(script_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?
        .code();

    Ok(code.unwrap_or(-1))
}
