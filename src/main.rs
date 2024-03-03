///////////////////////////////////////////////////////////////////////////////

use std::path::PathBuf;

use anyhow::Result;
use app::App;
use clap::Parser;

///////////////////////////////////////////////////////////////////////////////

mod app;

///////////////////////////////////////////////////////////////////////////////

fn main() {
    env_logger::init();

    let args = Config::parse();

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

#[derive(Parser)]
#[command(
    author="Aidan Beil",
    about="Graph demos", 
    long_about = Some("CS240 | Visual demo for BFS and DFS")
)]
pub struct Config {
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

///////////////////////////////////////////////////////////////////////////////
