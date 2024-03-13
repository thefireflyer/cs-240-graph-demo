///////////////////////////////////////////////////////////////////////////////

use std::fs;

use anyhow::Result;
use clap::Parser;
use cli::{Config, ExampleArgs, InteractiveArgs, NewArgs};
use cs_240_library::data_structures::graphs::{
    directed_graph::DirectedGraph, undirected_graph::UndirectedGraph, IGraph, IGraphEdgeMut,
    IGraphMut,
};

use gui::App;
use interactive::interactive;

///////////////////////////////////////////////////////////////////////////////

mod cli;
mod gui;
mod interactive;

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    env_logger::init();

    let args = Config::parse();

    match args.command {
        cli::Commands::New(args) => new(args),
        cli::Commands::Example(args) => example(args),
        cli::Commands::Gui => Ok(gui()),
        cli::Commands::Open(args) => open(args),
    }
}

///////////////////////////////////////////////////////////////////////////////

fn new(args: NewArgs) -> Result<()> {
    let mut graph = DirectedGraph::new();

    graph.insert_node("node-1");
    graph.insert_node("node-2");
    graph.insert_node("node-3");

    graph.insert_edge("node-1", "node-2");
    graph.insert_edge("node-1", "node-3");

    fs::write(args.path, serde_yaml::to_string(&graph)?)?;

    Ok(())
}

//---------------------------------------------------------------------------//

pub fn open(args: InteractiveArgs) -> Result<()> {
    let file_contents = fs::read_to_string(args.path.clone())?;
    let mut graph: DirectedGraph<String> = serde_yaml::from_str(&file_contents)?;

    interactive(&mut graph)?;

    fs::write(args.path, serde_yaml::to_string(&graph)?)?;

    Ok(())
}

//---------------------------------------------------------------------------//

fn example(args: ExampleArgs) -> Result<()> {
    println!("Opening in-memory example graph");
    println!();

    match args.example {
        cli::Example::Pathfinding => {
            let mut graph = UndirectedGraph::new();
            graph.insert_node("bellingham".to_owned());
            graph.insert_node("seattle".to_owned());
            graph.insert_node("everett".to_owned());
            graph.insert_node("arlington".to_owned());
            graph.insert_node("mt-vernon".to_owned());
            graph.insert_node("ferndale".to_owned());
            graph.insert_node("anacortes".to_owned());
            graph.insert_node("edmonds".to_owned());
            graph.insert_node("redmond".to_owned());
            graph.insert_node("seatac".to_owned());
            graph.insert_node("tacoma".to_owned());
            graph.insert_node("vancouver".to_owned());
            graph.insert_node("bothell".to_owned());

            graph.insert_edge("mt-vernon".to_owned(), "bellingham".to_owned());
            graph.insert_edge("bellingham".to_owned(), "ferndale".to_owned());
            graph.insert_edge("vancouver".to_owned(), "ferndale".to_owned());
            graph.insert_edge("mt-vernon".to_owned(), "anacortes".to_owned());
            graph.insert_edge("mt-vernon".to_owned(), "arlington".to_owned());
            graph.insert_edge("arlington".to_owned(), "everett".to_owned());
            graph.insert_edge("everett".to_owned(), "edmonds".to_owned());
            graph.insert_edge("everett".to_owned(), "bothell".to_owned());
            graph.insert_edge("edmonds".to_owned(), "bothell".to_owned());
            graph.insert_edge("edmonds".to_owned(), "seattle".to_owned());
            graph.insert_edge("bothell".to_owned(), "seattle".to_owned());
            graph.insert_edge("bothell".to_owned(), "redmond".to_owned());
            graph.insert_edge("seattle".to_owned(), "seatac".to_owned());
            graph.insert_edge("seatac".to_owned(), "tacoma".to_owned());

            println!("Try using the `route` command");
            println!("For example: `route Bellingham Redmond`");

            println!();

            let mut graph = DirectedGraph::from(graph);

            interactive(&mut graph)?;

            Ok(())
        }
        cli::Example::JobScheduling => {
            let mut graph = DirectedGraph::new();
            graph.insert_node("task-1".to_owned());
            graph.insert_node("task-2".to_owned());
            graph.insert_node("task-3".to_owned());
            graph.insert_node("task-4".to_owned());
            graph.insert_node("task-5".to_owned());
            graph.insert_node("task-6".to_owned());
            graph.insert_node("task-7".to_owned());
            graph.insert_node("task-8".to_owned());
            graph.insert_node("task-9".to_owned());
            graph.insert_node("task-10".to_owned());

            graph.insert_edge("task-1".to_owned(), "task-2".to_owned());
            graph.insert_edge("task-1".to_owned(), "task-3".to_owned());
            graph.insert_edge("task-1".to_owned(), "task-4".to_owned());

            graph.insert_edge("task-2".to_owned(), "task-5".to_owned());
            graph.insert_edge("task-2".to_owned(), "task-6".to_owned());

            graph.insert_edge("task-3".to_owned(), "task-7".to_owned());

            graph.insert_edge("task-7".to_owned(), "task-9".to_owned());
            graph.insert_edge("task-7".to_owned(), "task-10".to_owned());
            graph.insert_edge("task-6".to_owned(), "task-8".to_owned());

            println!("Try using the `schedule` command");
            println!();

            interactive(&mut graph)?;

            Ok(())
        }
    }
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
