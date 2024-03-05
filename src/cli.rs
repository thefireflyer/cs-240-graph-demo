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
    Open(FileArgs),
    OpenInteractive(InteractiveArgs),
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

//---------------------------------------------------------------------------//

#[derive(Args)]
pub struct FileArgs {
    #[arg(short, long)]
    pub path: PathBuf,

    #[command(subcommand)]
    pub command: FileCommands,
}

//...........................................................................//

#[derive(Args)]
pub struct InteractiveArgs {
    #[arg(short, long)]
    pub path: PathBuf,
}

//...........................................................................//

#[derive(Subcommand)]
pub enum FileCommands {
    List,
    Add(NodeArgs),
    Remove(NodeArgs),
    Connect(EdgeArgs),
    Disconnect(EdgeArgs),
    Inspect(NodeArgs),
    BFS(NodeArgs),
    DFS,
    TopologicalSort,
}

//...........................................................................//

#[derive(Args)]
pub struct NodeArgs {
    #[arg(short, long)]
    pub node: String,
}

//...........................................................................//

#[derive(Args)]
pub struct EdgeArgs {
    #[arg(short, long)]
    pub from: String,

    #[arg(short, long)]
    pub to: String,
}

///////////////////////////////////////////////////////////////////////////////
