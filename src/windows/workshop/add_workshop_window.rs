use chrono::NaiveDate;
use crate::database_logic::data_structs::Workshop;
use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

#[derive(Default)]
pub struct AddWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    name: String,
    facilitator: i32,
    venue: i32,
    address: (bool, String),
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl AddWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Create Venue")
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("create_venue_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name:");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
                            ui.end_row();
                            ui.label("Facilitator:");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
                            ui.end_row();
                            ui.label("Venue:");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
                            ui.end_row();
                            ui.add(
                                DatePickerButton::new(&mut self.start_date)
                                    .format("%d-%m-%Y")
                                    .highlight_weekends(false)
                                    .id_source("start_date"),
                            );
                            ui.end_row();
                            ui.add(
                                DatePickerButton::new(&mut self.end_date)
                                    .format("%d-%m-%Y")
                                    .highlight_weekends(false)
                                    .id_source("end_date"),
                            );
                            ui.end_row();
                        });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("➕ ADD").clicked() {
                        let new_workshop = Workshop {
                            id: None,
                            name: self.name.clone(),
                            facilitator: self.facilitator,
                            venue: self.venue,
                            start_date: self.start_date,
                            end_date: self.end_date,
                        };
                        self.db.add_workshop(new_workshop).unwrap();
                    };
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;
                    };
                });
            });
        if !self.open | self.reset {
            self.reset_values();
        }
    }

    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
