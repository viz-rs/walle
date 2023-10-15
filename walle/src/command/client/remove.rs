use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(infer_subcommands = true)]
pub struct Cmd {}

impl Cmd {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
