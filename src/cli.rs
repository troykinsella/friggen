use std::collections::HashMap;
use std::process::exit;

use crate::args::Args;
use crate::error::{FriggenError, Result};
use crate::friggen::Friggen;
use crate::fs_context::resolve_fs_context;
use crate::print::{OutputPrinter, PrintTheme};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub struct Cli {
    args: Args,
}

impl Cli {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    fn parse_env_vars(&self) -> HashMap<&str, &str> {
        self.args
            .env_var
            .iter()
            .map(|e| {
                if let Some(parts) = e.split_once('=') {
                    parts
                } else {
                    (e.as_str(), "")
                }
            })
            .collect()
    }

    fn create_friggen(&self) -> Result<Friggen> {
        let fs_context = resolve_fs_context(
            self.args.friggenfile.as_deref(),
            self.args.working_dir.as_deref(),
        )?;

        let output_printer = OutputPrinter::new(PrintTheme::ThisFriggenKicksAss, self.args.quiet);

        Ok(Friggen::new(
            fs_context,
            self.args.tasks.iter().map(|s| s.as_str()).collect(),
            self.parse_env_vars(),
            output_printer,
        ))
    }

    pub fn run(&self) {
        if self.args.version {
            println!("{} {}", APP_NAME, VERSION);
            return;
        }

        let friggen = match self.create_friggen() {
            Ok(f) => f,
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        };

        if let Err(err) = friggen.run() {
            match err {
                FriggenError::TaskError { task: _, exit_code } => {
                    // Message already printed in task summary, but make sure we:
                    exit(exit_code)
                }
                _ => {
                    eprintln!("{}", err);
                    exit(1);
                }
            }
        }
    }
}
