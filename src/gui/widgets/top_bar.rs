///////////////////////////////////////////////////////////////////////////////

use crate::gui::App;

///////////////////////////////////////////////////////////////////////////////

pub fn top_bar(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);

            ui.menu_button("File", |ui| {
                if ui.button("New Graph").clicked() {
                    app.new_graph();
                    ui.close_menu();
                }

                if ui.button("Open Graph").clicked() {
                    app.find_graph();
                    ui.close_menu();
                }

                ui.menu_button("Open Recent", |ui| {
                    for path in &app.recent {
                        if ui
                            .button(
                                path.file_name()
                                    .unwrap_or_default()
                                    .to_str()
                                    .unwrap_or_default(),
                            )
                            .clicked()
                        {
                            app.open_graph();
                            ui.close_menu();
                            ui.close_menu();
                            break;
                        }
                    }
                });

                ui.separator();

                if ui.button("Save").clicked() {
                    ui.close_menu();
                    todo!()
                }
                if ui.button("Save As ...").clicked() {
                    ui.close_menu();
                    todo!()
                }

                ui.separator();

                if ui.button("Close Graph").clicked() {
                    app.close_graph();
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    });
}

///////////////////////////////////////////////////////////////////////////////
