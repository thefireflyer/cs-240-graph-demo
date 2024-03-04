///////////////////////////////////////////////////////////////////////////////

use egui::{
    ahash::{HashMap, HashMapExt, HashSet, HashSetExt},
    emath::TSTransform,
    Color32, Pos2, Stroke, Vec2,
};
use std::{f32::consts::PI, path::PathBuf};

use cs_240_library::data_structures::graphs::{undirected_graph::UndirectedGraph, Graph, GraphMut};

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    active: Option<PathBuf>,
    recent: Vec<PathBuf>,

    #[serde(skip)]
    graph_data: Option<GraphData>,
    #[serde(skip)]
    graph_text: String,
    pan_zoom: PanZoom,
}

//---------------------------------------------------------------------------//

impl Default for App {
    fn default() -> Self {
        Self {
            active: None,
            recent: vec![],
            graph_data: None,
            graph_text: "".to_owned(),
            pan_zoom: Default::default(),
        }
    }
}

//---------------------------------------------------------------------------//

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    //-----------------------------------------------------------------------//

    fn new_graph(&mut self) {
        let mut graph = GraphData::new();
        for i in 0..20 {
            graph.add_node(i.to_string());
        }
        graph.add_connection("0".to_owned(), "2".to_owned());
        graph.reposition();

        self.graph_data = Some(graph.clone());
        self.graph_text = serde_yaml::to_string(&graph).expect("test");
    }

    //-----------------------------------------------------------------------//

    fn open_graph(&mut self, project: &PathBuf) {
        todo!()
    }

    //-----------------------------------------------------------------------//

    fn save_graph(&mut self) {
        if let Some(project) = &self.active {
            self.save_graph_to(project.clone());
        } else {
            todo!()
        }
    }

    //-----------------------------------------------------------------------//

    fn save_graph_to(&mut self, path: PathBuf) {
        todo!()
    }

    //-----------------------------------------------------------------------//

    fn close_graph(&mut self) {
        todo!()
    }

    //-----------------------------------------------------------------------//

    fn update_graph(&mut self) {
        let new_data: Result<GraphData, serde_yaml::Error> = serde_yaml::from_str(&self.graph_text);
        if let Ok(data) = new_data {
            self.graph_data = Some(data);
            println!("updating");
        } else {
            println!("failed to parse: {:?}", new_data);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.menu_button("File", |ui| {
                    if ui.button("New Graph").clicked() {
                        self.new_graph();
                        ui.close_menu();
                    }

                    if ui.button("Open Graph").clicked() {
                        todo!()
                    }

                    ui.menu_button("Open Recent", |ui| {
                        for project in &self.recent.clone() {
                            if ui
                                .button(project.file_name().unwrap().to_str().unwrap())
                                .clicked()
                            {
                                self.open_graph(project);
                                ui.close_menu();
                            }
                        }
                    });

                    ui.separator();

                    if ui.button("Save").clicked() {
                        todo!()
                    }
                    if ui.button("Save As ...").clicked() {
                        todo!()
                    }

                    ui.separator();

                    if ui.button("Close Graph").clicked() {
                        self.close_graph();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        if let Some(graph) = &mut self.graph_data {
            egui::SidePanel::left("left_panel")
                .default_width(320.0)
                .min_width(240.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if ui.code_editor(&mut self.graph_text).changed() {
                            // self.update_graph();
                        }
                    });
                });

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Graph");
                self.pan_zoom
                    .ui(ui, &graph.positions, &graph.adjacency_list);
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("No project open");
                })
            });
        }
    }

    //-----------------------------------------------------------------------//

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GraphData {
    adjacency_list: UndirectedGraph<String>,
    positions: HashMap<String, Pos2>,
    roots: HashSet<String>,
}

//---------------------------------------------------------------------------//

impl GraphData {
    pub fn new() -> Self {
        GraphData {
            adjacency_list: UndirectedGraph::new(),
            positions: HashMap::new(),
            roots: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: String) {
        self.adjacency_list.insert_node(node.clone(), vec![]);
        self.roots.insert(node.clone());
        self.positions.insert(
            node,
            Pos2 {
                x: 200.0 * f32::from(i16::try_from(self.roots.len()).unwrap()),
                y: 100.0,
            },
        );
    }

    pub fn add_connection(&mut self, from: String, to: String) {
        self.roots.remove(&from);
        self.roots.remove(&to);
        self.adjacency_list.insert_edge(from, to);
    }

    pub fn reposition(&mut self) {
        let len = (typing(self.positions.len()) - 1.0);
        for (i, (node, pos)) in self.positions.iter_mut().enumerate() {
            *pos = Pos2 {
                x: (2.0 * PI * (typing(i)) / len * 4.0).cos() * 6.0 * len + 300.0,
                y: (2.0 * PI * typing(i) / len * 4.0).sin() * 6.0 * len + 300.0,
            }
        }
    }
}

fn typing(x: usize) -> f32 {
    f32::from(i16::try_from(x).unwrap())
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PanZoom {
    transform: TSTransform,
    drag_value: f32,
}

///////////////////////////////////////////////////////////////////////////////

impl PanZoom {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        graph: &HashMap<String, Pos2>,
        adj: &UndirectedGraph<String>,
    ) {
        ui.label(
            "Pan, zoom in, and zoom out with scrolling (see the plot demo for more instructions). \
               Double click on the background to reset.",
        );
        ui.separator();

        let (id, rect) = ui.allocate_space(ui.available_size());
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());
        // Allow dragging the background as well.
        if response.dragged() {
            self.transform.translation += response.drag_delta();
        }

        // Plot-like reset
        if response.double_clicked() {
            self.transform = TSTransform::default();
        }

        let transform =
            TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * self.transform;

        if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
            // Note: doesn't catch zooming / panning if a button in this PanZoom container is hovered.
            if response.hovered() {
                let pointer_in_layer = transform.inverse() * pointer;
                let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);

                // Zoom in on pointer:
                self.transform = self.transform
                    * TSTransform::from_translation(pointer_in_layer.to_vec2())
                    * TSTransform::from_scaling(zoom_delta)
                    * TSTransform::from_translation(-pointer_in_layer.to_vec2());

                // Pan:
                self.transform = TSTransform::from_translation(pan_delta) * self.transform;
            }
        }

        for (i, (node, pos)) in graph.iter().enumerate() {
            let id = egui::Area::new(id.with(("subarea", i)))
                .fixed_pos(*pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    egui::Frame::default()
                        .rounding(egui::Rounding::same(40.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .stroke(ui.ctx().style().visuals.window_stroke)
                        .fill(ui.style().visuals.panel_fill)
                        .show(ui, |ui| {
                            ui.style_mut().wrap = Some(false);

                            ui.label(node);

                            let painter = ui.painter();

                            for adj in adj.get_adj(&node) {
                                painter.line_segment(
                                    [
                                        *graph.get(node).unwrap() + Vec2::new(8.0, 8.0),
                                        *graph.get(&adj).unwrap() + Vec2::new(8.0, 8.0),
                                    ],
                                    Stroke::new(1.0, Color32::DARK_GRAY),
                                );
                            }
                        });
                })
                .response
                .layer_id;
            ui.ctx().set_transform_layer(id, transform);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
