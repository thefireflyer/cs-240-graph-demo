///////////////////////////////////////////////////////////////////////////////

use std::{
    fs,
    io::{self, Stdin, Write},
    path::PathBuf,
};

use anyhow::Result;
use app::App;
use clap::{Args, Parser, Subcommand};
use cli::{Config, ExampleArgs, FileArgs, InteractiveArgs, NewArgs};
use cs_240_library::data_structures::graphs::{
    breadth_first_search, undirected_graph::UndirectedGraph, Graph, GraphMut,
};

use crate::cli::{EdgeArgs, NodeArgs};
use interactive::interactive;

///////////////////////////////////////////////////////////////////////////////

mod app;
mod cli;
mod interactive;

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    env_logger::init();

    let args = Config::parse();

    match args.command {
        cli::Commands::New(args) => new(args),
        cli::Commands::Open(args) => open(args),
        cli::Commands::Example(args) => example(args),
        cli::Commands::Gui => Ok(gui()),
        cli::Commands::OpenInteractive(args) => interactive(args),
    }
}

///////////////////////////////////////////////////////////////////////////////

fn new(args: NewArgs) -> Result<()> {
    let mut graph: UndirectedGraph<&str> = UndirectedGraph::new();

    graph.insert_node("Example node 1", vec![]);
    graph.insert_node("Example node 2", vec![]);
    graph.insert_node("Example node 3", vec![]);

    graph.insert_edge("Example node 1", "Example node 2");
    graph.insert_edge("Example node 1", "Example node 3");

    fs::write(args.path, serde_yaml::to_string(&graph)?)?;

    Ok(())
}

//---------------------------------------------------------------------------//

fn open(args: FileArgs) -> Result<()> {
    let file_contents = fs::read_to_string(args.path.clone())?;
    let mut graph: UndirectedGraph<String> = serde_yaml::from_str(&file_contents)?;

    match args.command {
        cli::FileCommands::List => list(&graph),
        cli::FileCommands::Add(args) => add(args, &mut graph),
        cli::FileCommands::Remove(args) => remove(args, &mut graph),
        cli::FileCommands::Connect(args) => connect(args, &mut graph),
        cli::FileCommands::Disconnect(args) => disconnect(args, &mut graph),
        cli::FileCommands::Inspect(args) => inspect(args, &graph),
        cli::FileCommands::BFS(args) => bfs(args, &graph),
        cli::FileCommands::DFS => dfs(&graph),
        cli::FileCommands::TopologicalSort => topo_sort(&graph),
    }?;

    fs::write(args.path, serde_yaml::to_string(&graph)?)?;

    Ok(())
}

//---------------------------------------------------------------------------//

fn example(args: ExampleArgs) -> Result<()> {
    todo!()
}

///////////////////////////////////////////////////////////////////////////////

fn gui() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 540.0])
            .with_min_inner_size([960.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Graph Demo",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .expect("Failed to run eframe");
}

///////////////////////////////////////////////////////////////////////////////

fn list(graph: &UndirectedGraph<String>) -> Result<()> {
    let mut nodes = graph.get_all();
    nodes.sort();

    for node in nodes {
        println!("- {}", node);
    }

    Ok(())
}

fn add(args: NodeArgs, graph: &mut UndirectedGraph<String>) -> Result<()> {
    graph.insert_node(args.node, vec![]);
    Ok(())
}

fn remove(args: NodeArgs, graph: &mut UndirectedGraph<String>) -> Result<()> {
    graph.remove_node(args.node);
    Ok(())
}

fn connect(args: EdgeArgs, graph: &mut UndirectedGraph<String>) -> Result<()> {
    graph.insert_edge(args.from, args.to);
    Ok(())
}

fn disconnect(args: EdgeArgs, graph: &mut UndirectedGraph<String>) -> Result<()> {
    graph.remove_edge(args.from, args.to);
    Ok(())
}

fn inspect(args: NodeArgs, graph: &UndirectedGraph<String>) -> Result<()> {
    println!("{}", args.node);
    for node in graph.get_adj(&args.node) {
        println!("    - {}", node);
    }
    Ok(())
}

fn bfs(args: NodeArgs, graph: &UndirectedGraph<String>) -> Result<()> {
    let mapping = breadth_first_search(graph.clone(), args.node.clone());
    let mut mapping: Vec<(&String, &Vec<String>)> = mapping.iter().collect();

    mapping.sort_by_key(|(_, path)| path.len());

    for (node, path) in mapping {
        if node != &args.node {
            for node in path {
                print!("{} -> ", node);
            }
            println!("{}", node);
        }
    }

    Ok(())
}

fn dfs(graph: &UndirectedGraph<String>) -> Result<()> {
    todo!()
}

fn topo_sort(graph: &UndirectedGraph<String>) -> Result<()> {
    todo!()
}

///////////////////////////////////////////////////////////////////////////////
