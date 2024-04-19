use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone, Debug)]
#[command(name = "friggen")]
#[command(about = "A friggen task runner.")]
pub struct Args {
    /// Print the agent version and exit.
    #[arg(short = 'v', long)]
    pub version: bool,

    /// The path to the friggenfile.
    #[arg(short = 'f', long, env = "FRIGGEN_FILE", value_name = "PATH")]
    pub friggenfile: Option<PathBuf>,

    /// The path to the working directory.
    #[arg(short = 'w', long, env = "FRIGGEN_WORKING_DIR", value_name = "PATH")]
    pub working_dir: Option<PathBuf>,

    /// Supply an environment variable to task scripts.
    #[arg(short = 'e', long, value_name = "NAME=VALUE")]
    pub env_var: Vec<String>,

    /// Only print task output.
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Names of tasks to run. Run with no arguments to list available tasks and task help.
    #[arg()]
    pub tasks: Vec<String>,
}
