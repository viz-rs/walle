use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(infer_subcommands = true)]
pub enum Cmd {}

impl Cmd {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
