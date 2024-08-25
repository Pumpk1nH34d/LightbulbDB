use chrono::NaiveDate;
use crate::database_logic::data_structs::Workshop;
use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

#[derive(Default)]
pub struct EditWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    changed: bool,
    workshop_check: Workshop,

    name: String,
    facilitator: String,
    venue: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl EditWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context, workshop: Workshop) {
        if self.workshop_check.id != workshop.id {
            self.workshop_check = workshop.clone();
            self.facilitator = workshop.facilitator.to_string();
            self.venue = workshop.venue.to_string();
            self.start_date = workshop.start_date;
            self.end_date = workshop.end_date;
        }
        if !self.open {
            self.changed = false;
        };
        egui::Window::new("Edit Workshop")
            .open(&mut self.open)
            .show(ctx, |ui| {
                if self.changed {
                    ui.label("DONE ✔");
                    ui.label("Reselect Workshop to see changes.");
                } else if self.workshop_check.id.is_none() {
                    ui.label("Select a Workshop to EDIT");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("edit_workshop")
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
                        if ui.button("✔ Confirm").clicked() {
                            let edited_workshop = Workshop {
                                id: self.workshop_check.id,
                                name: self.name.clone(),
                                facilitator: self.facilitator.clone().parse::<i32>().unwrap_or(0),
                                venue: self.venue.clone().parse::<i32>().unwrap_or(0),
                                start_date: self.start_date.clone(),
                                end_date: self.end_date.clone(),
                            };
                            self.db.edit_workshop(edited_workshop).unwrap();
                            self.workshop_check.id = None;
                            self.changed = true;
                        };
                        if ui.button("❌ Delete").clicked() {
                            self.db.delete_workshop(self.workshop_check.id.unwrap());
                            self.workshop_check.id = None;
                            self.changed = true;
                        };
                    });
                }
            });
    }
}
