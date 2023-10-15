use anyhow::Result;
use clap::Subcommand;

mod add;
mod list;
mod remove;

#[derive(Subcommand, Debug)]
#[command(infer_subcommands = true)]
pub enum Cmd {
    /// Add a project to tracking
    Add(add::Cmd),

    /// Remove a project from tracking list
    Remove(remove::Cmd),

    /// List all projects being tracked
    List(list::Cmd),
}

impl Cmd {
    pub fn run(self) -> Result<()> {
        match self {
            Self::Add(cmd) => cmd.run(),
            Self::Remove(cmd) => cmd.run(),
            Self::List(cmd) => cmd.run(),
        }
    }
}
