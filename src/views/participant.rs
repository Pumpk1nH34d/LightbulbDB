use egui::{Context, Ui};
use crate::database::DataBase;

#[derive(Default)]
pub struct ParticipantsView {
    add_window_open: bool,
    edit_window_open: bool,
    reset_window_open: bool,
    db: DataBase,
}

impl ParticipantsView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.label("TEST");
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.windows_view(ui, ctx);
    }

    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Austin Delic");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Right Panel!");
                });
            });
    }

    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(15.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add").clicked() {
                        self.db.drop_db().unwrap();
                        self.db.create_db().unwrap();
                        self.add_window_open = !self.add_window_open;
                    };
                    if ui.button("Edit").clicked() {
                        self.edit_window_open = !self.edit_window_open;
                    };
                    if ui.button("RESET").clicked() {
                        self.reset_window_open = !self.reset_window_open;
                    };
                });
            });
    }
    fn windows_view(&mut self, ui: &mut Ui, ctx: &Context) {
        if self.add_window_open {
            self.add_window(ui, ctx);
        }
        if self.edit_window_open {
            self.edit_window(ui, ctx);
        }
    }

    fn add_window(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Add")
            .open(&mut self.add_window_open)
            .show(ctx, |ui| {
                ui.label("done");
            });
    }

    fn edit_window(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Edit")
            .open(&mut self.edit_window_open)
            .show(ctx, |ui| {
                ui.label("LOL");
            });
    }
}
