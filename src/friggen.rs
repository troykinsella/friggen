use std::borrow::Cow;
use std::collections::HashMap;
use std::time::SystemTime;

use colored::Colorize;

use crate::ast::AstNode;
use crate::error::{FriggenError, Result};
use crate::friggenfile::{Friggenfile, Task, TaskDep};
use crate::fs_context::FsContext;
use crate::ioutil::read_file;
use crate::print::{OutputPrinter, PrintTheme};
use crate::shell::{eval_shell_command, run_shell_script};

pub struct Friggen<'a> {
    fs_context: FsContext,
    tasks: Vec<&'a str>,
    env_vars: HashMap<&'a str, &'a str>,
    output_printer: OutputPrinter,
}

impl<'a> Friggen<'a> {
    const DEFAULT_HASH_BANG: [&'static str; 2] = ["/usr/bin/env", "bash"];

    pub fn new(
        fs_context: FsContext,
        tasks: Vec<&'a str>,
        env_vars: HashMap<&'a str, &'a str>,
        output_printer: OutputPrinter,
    ) -> Self {
        Self {
            fs_context,
            tasks,
            env_vars,
            output_printer,
        }
    }

    pub fn run(&self) -> Result<()> {
        let start_time = SystemTime::now();

        let buf = read_file(&self.fs_context.friggenfile)?;
        let buf = String::from_utf8_lossy(&buf);
        let ff = Friggenfile::from(&buf)?;
        log::debug!("ast: {:?}", ff.ast());

        let mut tasks: HashMap<&str, Task<'_>> = HashMap::new();
        build_task_map(ff.ast(), &mut tasks)?;
        validate_tasks(&tasks)?;

        let mut vars: HashMap<&str, Cow<'_, str>> = HashMap::new();
        self.build_var_map(ff.ast(), &mut vars)?;
        log::debug!("vars: {:?}", vars);

        if self.tasks.is_empty() {
            self.print_docs(&tasks);
            return Ok(());
        }

        let task_seq = build_task_sequence(&self.tasks, &tasks)?;
        log::debug!("sequence: {:?}", task_seq);

        let mut last_task: &str = "";
        let mut last_code: i32 = 0;
        for task_name in task_seq {
            last_task = task_name;
            last_code = self.run_task(task_name, &tasks, &vars)?;
            if last_code != 0 {
                break;
            }
        }

        self.output_printer
            .with_theme(print_theme_for_code(last_code))
            .print_timed_header("★ done", start_time);

        if last_code != 0 {
            return Err(FriggenError::TaskError {
                task: last_task.to_string(),
                exit_code: last_code,
            });
        }

        Ok(())
    }

    fn print_docs(&self, tasks: &HashMap<&str, Task<'_>>) {
        let mut tasks: Vec<&Task> = tasks.values().collect();
        tasks.sort_by(|a, b| a.name.partial_cmp(b.name).unwrap());

        println!(
            "{}",
            r#"
┏  •
╋┏┓┓┏┓┏┓┏┓┏┓
┛┛ ┗┗┫┗┫┗ ┛┗
     ┛ ┛
        "#
            .bright_purple()
        );

        println!("{}", "friggen tasks:".yellow().bold());
        println!(
            "{}",
            tasks
                .iter()
                .map(|task| task.name.bright_blue().bold().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!();

        for task in tasks {
            self.output_printer.print_section_header(task.name);

            if !task.deps.is_empty() {
                let color_deps: Vec<_> = task
                    .deps
                    .iter()
                    .map(|dep| dep.name.blue().bold().to_string())
                    .collect();
                self.output_printer.print_section_line(&format!(
                    "{} {}",
                    "»".purple(),
                    color_deps.join(" » ").purple()
                ));
            }

            if let Some(docs) = &task.docs {
                for line in docs {
                    self.output_printer.print_section_line(line.trim());
                }
            }

            self.output_printer.print_section_footer();
        }
    }

    fn run_task(
        &self,
        task_name: &str,
        tasks: &HashMap<&str, Task<'_>>,
        vars: &HashMap<&str, Cow<'_, str>>,
    ) -> Result<i32> {
        let start = SystemTime::now();

        let task = tasks.get(task_name).expect("task name exists");
        self.output_printer
            .print_header(&format!("» start: {}", task_name));

        let default_hash_bang = Vec::from(Self::DEFAULT_HASH_BANG);
        let hash_bang = task.hash_bang.as_ref().unwrap_or(&default_hash_bang);

        let code = run_shell_script(hash_bang, &task.script, &self.env_vars, vars)?;
        let msg = if code == 0 {
            format!("✓ done: {}", task_name)
        } else {
            format!("✗ failed: {} ({})", task_name, code)
        };

        self.output_printer
            .with_theme(print_theme_for_code(code))
            .print_timed_header(&msg, start);

        Ok(code)
    }

    fn build_var_map(
        &self,
        el: &'a AstNode,
        vars: &mut HashMap<&'a str, Cow<'a, str>>,
    ) -> Result<()> {
        match el {
            AstNode::Root(body) => {
                for el in body {
                    self.build_var_map(el, vars)?;
                }
            }
            AstNode::VarAssignment(var) => {
                let name = var.name;
                let value = match var.value.as_ref() {
                    AstNode::VarValue(value) => Cow::from(*value),
                    AstNode::CommandSubstitution(command) => {
                        let output = eval_shell_command("bash", command, &self.env_vars)?;
                        // Mimic shell behaviour of removing trailing newlines in command substitution
                        let output = output.trim_end_matches(&['\r', '\n']).to_string();
                        Cow::from(output)
                    }
                    _ => unreachable!(),
                };
                vars.insert(name, value);
            }
            _ => {}
        }
        Ok(())
    }
}

#[inline]
fn print_theme_for_code(code: i32) -> PrintTheme {
    if code == 0 {
        PrintTheme::ThisFriggenKicksAss
    } else {
        PrintTheme::ThisFriggenSucks
    }
}

fn build_task_map<'a>(el: &'a AstNode, tasks: &mut HashMap<&'a str, Task<'a>>) -> Result<()> {
    match el {
        AstNode::Root(body) => {
            for el in body {
                build_task_map(el, tasks)?;
            }
        }
        AstNode::TaskDef(def) => {
            let header = def.header.as_task_header();
            let deps: Vec<TaskDep> = header
                .deps
                .iter()
                .map(|dep| {
                    let dep = dep.as_task_dep();
                    TaskDep {
                        name: dep.name,
                        run_always: dep.run_always,
                    }
                })
                .collect();
            let docs = def.docs.as_ref().map(|docs| docs.as_task_docs().clone());

            let script = def.script.as_task_script();

            let task_name = header.name;

            // Ugh: https://github.com/rust-lang/rust/issues/82766
            if tasks.contains_key(task_name) {
                return Err(FriggenError::DuplicateTaskDefinition(task_name.to_string()));
            }
            tasks.insert(
                task_name,
                Task {
                    name: header.name,
                    deps,
                    docs,
                    hash_bang: script.hash_bang.clone(),
                    script: script.lines.clone(),
                },
            );
        }
        _ => {}
    }
    Ok(())
}

fn validate_tasks(tasks: &HashMap<&str, Task<'_>>) -> Result<()> {
    for task in tasks.values() {
        for dep in &task.deps {
            if !tasks.contains_key(dep.name) {
                return Err(FriggenError::InvalidTaskReference {
                    referrer: task.name.to_string(),
                    referee: dep.name.to_string(),
                });
            }
        }
    }
    Ok(())
}

fn build_task_sequence<'a>(
    requested_tasks: &'a [&str],
    tasks: &'a HashMap<&str, Task<'_>>,
) -> Result<Vec<&'a str>> {
    let mut seq: Vec<&str> = Vec::with_capacity(32);
    let mut stack: Vec<&str> = Vec::with_capacity(16);

    for task_name in requested_tasks {
        resolve_task_sequence(task_name, tasks, &mut seq, &mut stack)?;
        if !seq.contains(task_name) {
            seq.push(task_name);
        }
    }

    Ok(seq)
}

fn resolve_task_sequence<'a>(
    task_name: &'a str,
    tasks: &'a HashMap<&'a str, Task<'_>>,
    seq: &mut Vec<&'a str>,
    stack: &mut Vec<&'a str>,
) -> Result<()> {
    if stack.contains(&task_name) {
        stack.push(task_name);
        return Err(FriggenError::CyclicTaskReference(
            stack.clone().iter().map(|name| name.to_string()).collect(),
        ));
    }

    stack.push(task_name);

    let task = tasks
        .get(task_name)
        .ok_or_else(|| FriggenError::TaskNotFound(task_name.to_string()))?;

    for dep in &task.deps {
        resolve_task_sequence(dep.name, tasks, seq, stack)?;
        if dep.run_always || !seq.contains(&dep.name) {
            seq.push(dep.name);
        }
    }

    let top = stack.pop();
    assert!(top.is_some());

    Ok(())
}
