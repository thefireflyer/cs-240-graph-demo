///////////////////////////////////////////////////////////////////////////////

use cs_240_library::data_structures::graphs::{
    directed_graph::DirectedGraph, undirected_graph::UndirectedGraph,
    weighted_graph::WeightedGraph, IGraph,
};
use egui::{ahash::HashMap, emath::TSTransform, Color32, LayerId, Pos2, Stroke, Vec2};

use crate::gui::{
    pages::{PanZoom, Project},
    App,
};

///////////////////////////////////////////////////////////////////////////////

pub fn page_view(app: &mut App, ctx: &egui::Context) {
    match &mut app.active {
        crate::gui::pages::Page::Blank => blank_view(ctx),
        crate::gui::pages::Page::Project(project) => project_view(ctx, project),
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn blank_view(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.centered_and_justified(|ui| {
            ui.heading("No project open");
        })
    });
}

///////////////////////////////////////////////////////////////////////////////

pub fn project_view(ctx: &egui::Context, project: &mut Project) {
    egui::SidePanel::left("left_panel")
        .default_width(320.0)
        .min_width(240.0)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if ui.code_editor(&mut project.text).changed() {
                    project.update_graph();
                }
            });
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        pan_zoom_view(&mut project.view, ui, &project.graphic, &project.graph);
    });
}

///////////////////////////////////////////////////////////////////////////////

fn pan_zoom_view(
    view: &mut PanZoom,
    ui: &mut egui::Ui,
    graph: &HashMap<String, Pos2>,
    adj: &WeightedGraph<String, i32>,
) {
    ui.label(
        "Pan, zoom in, and zoom out with scrolling. \
               Double click on the background to reset.",
    );
    ui.separator();

    let (id, rect) = ui.allocate_space(ui.available_size());
    let response = ui.interact(rect, id, egui::Sense::click_and_drag());
    // Allow dragging the background as well.
    if response.dragged() {
        view.transform.translation += response.drag_delta();
    }

    // Plot-like reset
    if response.double_clicked() {
        view.transform = TSTransform::default();
    }

    let transform =
        TSTransform::from_translation(ui.min_rect().left_top().to_vec2()) * view.transform;

    if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
        // Note: doesn't catch zooming / panning if a button in this PanZoom container is hovered.
        if response.hovered() {
            let pointer_in_layer = transform.inverse() * pointer;
            let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
            let pan_delta = ui.ctx().input(|i| i.smooth_scroll_delta);

            // Zoom in on pointer:
            view.transform = view.transform
                * TSTransform::from_translation(pointer_in_layer.to_vec2())
                * TSTransform::from_scaling(zoom_delta)
                * TSTransform::from_translation(-pointer_in_layer.to_vec2());

            // Pan:
            view.transform = TSTransform::from_translation(pan_delta) * view.transform;
        }
    }

    for (i, (node, pos)) in graph.iter().enumerate() {
        {
            let id = egui::Area::new(id.with(("bg", i)))
                .fixed_pos(*pos)
                .order(egui::Order::Background)
                .show(ui.ctx(), |ui| {
                    ui.set_clip_rect(transform.inverse() * rect);
                    egui::Frame::default()
                        .rounding(egui::Rounding::same(40.0))
                        .inner_margin(egui::Margin::same(8.0))
                        .stroke(ui.ctx().style().visuals.window_stroke)
                        .fill(ui.style().visuals.panel_fill)
                        .show(ui, |ui| {
                            ui.label(node);
                            let painter = ui.painter();

                            for adj in adj.get_adj(&node) {
                                painter.line_segment(
                                    [
                                        *graph.get(node).unwrap() + Vec2::new(20.0, 16.0),
                                        *graph.get(&adj).unwrap() + Vec2::new(20.0, 16.0),
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
                    });
            })
            .response
            .layer_id;

        ui.ctx().set_transform_layer(id, transform);
    }
}

///////////////////////////////////////////////////////////////////////////////
