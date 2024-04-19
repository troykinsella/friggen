use std::{env, fs};
use std::path::{Path, PathBuf};

use crate::error::{FriggenError, Result};

pub(crate) const FRIGGENFILE_NAMES: &[&str] = &["friggenfile", ".friggenfile"];
pub const PROJECT_ROOT_INDICATORS: &[&str] = &[".bzr", ".git", ".hg", ".svn", "_darcs"];

pub struct FsContext {
    pub friggenfile: PathBuf,
    pub working_dir: PathBuf,
}

pub fn resolve_fs_context(
    supplied_ff_path: Option<&Path>,
    supplied_wd_path: Option<&Path>,
) -> Result<FsContext> {
    match (supplied_ff_path, supplied_wd_path) {
        (Some(ff), Some(wd)) => Ok(FsContext {
            friggenfile: PathBuf::from(ff),
            working_dir: PathBuf::from(wd),
        }),
        (Some(ff), None) => Ok(FsContext {
            friggenfile: PathBuf::from(ff),
            working_dir: env::current_dir()?,
        }),
        (None, Some(wd)) => Ok(FsContext {
            friggenfile: find_friggenfile(wd)?,
            working_dir: PathBuf::from(wd),
        }),
        _ => {
            let wd = env::current_dir()?;
            let ff = find_friggenfile(&wd)?;
            Ok(FsContext {
                friggenfile: ff,
                working_dir: wd,
            })
        }
    }
}

fn find_friggenfile(dir: &Path) -> Result<PathBuf> {
    let ancestors = dir.ancestors();

    for dir in ancestors {
        if let Some(ff) = contains_friggenfile(dir)? {
            return Ok(ff);
        }
        if is_project_root(dir)? {
            // We're at the project root, but we didn't find the friggen friggenfile ffs
            break;
        }
    }

    Err(FriggenError::FriggenfileNotFound)
}

#[inline]
fn contains_friggenfile(dir: &Path) -> Result<Option<PathBuf>> {
    // Look for the friggen friggenfile, unkay?
    for name in FRIGGENFILE_NAMES {
        let path = dir.join(name);
        if fs::metadata(&path).is_ok() {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

#[inline]
fn is_project_root(dir: &Path) -> Result<bool> {
    for name in PROJECT_ROOT_INDICATORS {
        let path = dir.join(name);
        if fs::metadata(&path).is_ok() {
            return Ok(true);
        }
    }
    Ok(false)
}
