///////////////////////////////////////////////////////////////////////////////

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

///////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(
    author="Aidan Beil",
    about="Graph demos", 
    long_about = Some("CS240 | Demo for BFS and DFS")
)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

//---------------------------------------------------------------------------//

#[derive(Subcommand)]
pub enum Commands {
    New(NewArgs),
    Open(InteractiveArgs),
    Example(ExampleArgs),
    Gui,
}

//---------------------------------------------------------------------------//

#[derive(Args)]
pub struct NewArgs {
    #[arg(short, long)]
    pub path: PathBuf,
}

//---------------------------------------------------------------------------//

#[derive(Args)]
pub struct ExampleArgs {
    #[command(subcommand)]
    pub example: Example,
}

//...........................................................................//

#[derive(Subcommand)]
pub enum Example {
    Pathfinding,
    JobScheduling,
}

//...........................................................................//

#[derive(Args)]
pub struct InteractiveArgs {
    #[arg(short, long)]
    pub path: PathBuf,
}

///////////////////////////////////////////////////////////////////////////////
