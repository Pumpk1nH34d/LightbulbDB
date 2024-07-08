use crate::database::DataBase;
use egui::{Context, Ui};

pub struct AddWindow {
    pub open: bool,
    first_name: String,
    last_name: String,
}

impl AddWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Create Participant")
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::Grid::new("my_grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("First Name");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.first_name)
                                .hint_text("First Name"),
                        );
                        ui.end_row();
                        ui.label("Last Name");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.last_name)
                                .hint_text("Last Name"),
                        );
                        ui.end_row();
                    });
            });
        if !self.open {
            self.reset();
        }
    }
    
    pub fn reset(&mut self) {
        self.first_name = String::new();
        self.last_name = String::new();
        self.open = false;
    }
}

impl Default for AddWindow {
    fn default() -> Self {
        AddWindow {
            open: false,
            first_name: String::new(),
            last_name: String::new(),
        }
    }
}