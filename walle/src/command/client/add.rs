use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct Cmd {
    /// The path of project [default: "."]
    path: Option<String>,

    /// Maximum depth to scan
    #[arg(long, short, default_value_t = 1)]
    level: usize,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
