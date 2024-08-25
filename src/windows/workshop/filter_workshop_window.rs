use chrono::NaiveDate;
use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};

#[derive(Default)]
pub struct FilterWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    filter: String,

    name: String,
    facilitator: String,
    venue: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl FilterWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) -> String {
        egui::Window::new("Filer Workshop")
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("filter_workshop_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name:");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
                            ui.end_row();
                            ui.label("address:");
                            ui.add(TextEdit::singleline(&mut self.facilitator).hint_text("address"));
                            ui.end_row();
                            ui.label("suburb:");
                            ui.add(TextEdit::singleline(&mut self.venue).hint_text("suburb"));
                            ui.end_row();
                            ui.label("postcode:");
                        });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("✔ APPLY").clicked() {
                        let mut filter = String::new();
                        if !self.name.is_empty() {
                            filter += &format!("name = '{}', ", self.name)
                        };
                        if !self.facilitator.is_empty() {
                            filter += &format!("address = '{}', ", self.facilitator)
                        }
                        if !self.venue.is_empty() {
                            filter += &format!("suburb = '{}', ", self.venue)
                        }
                        if !filter.is_empty() {
                            filter.truncate(filter.len() - 2)
                        }
                        self.filter = filter;
                    }
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;
                    };
                });
            });
        if self.reset {
            self.reset_values();
        };
        self.filter.clone()
    }

    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
