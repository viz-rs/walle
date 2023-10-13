use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    /// Add a project to tracking
    Add,
    /// Remove a project from tracking list
    Remove,
    /// List all projects being tracked
    List,
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
