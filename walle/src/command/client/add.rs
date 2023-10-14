use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct Cmd {
    /// The path of project
    path: String,

    /// Maximum depth to scan
    #[arg(long, short, default_value = "1")]
    level: usize,
}

impl Cmd {
    pub fn run() -> Result<()> {
        Ok(())
    }
}
