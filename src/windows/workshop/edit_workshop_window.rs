use chrono::NaiveDate;
use crate::database_logic::data_structs::{SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

#[derive(Default)]
pub enum ForeignView {
    #[default]
    Facilitator,
    Venue,
}

#[derive(Default)]
pub struct EditWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,
    
    right_view_selected: Option<ForeignView>,
    changed: bool,
    workshop_check: Workshop,
    

    name: String,
    facilitator: SupportWorker,
    facilitator_filter: String,
    venue: Venue,
    venue_filter: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl EditWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context, workshop: Workshop) {
        if self.workshop_check.id != workshop.id {
            self.workshop_check = workshop.clone();
            self.facilitator = self.db.get_filtered_support_workers(format!("id = '{}'", workshop.facilitator))[0].clone();
            self.venue = self.db.get_filtered_venues(format!("id = '{}'", workshop.venue))[0].clone();
            self.name = workshop.name;
            self.start_date = workshop.start_date;
            self.end_date = workshop.end_date;
        }
        if !self.open {
            self.changed = false;
        };
        egui::Window::new("Edit Workshop")
            .open(&mut self.open)
            .max_height(110.0)
            .show(ctx, |ui| {
                egui::SidePanel::right("create_workshop_right_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("foreign_edit_filter_results")
                                .num_columns(6)
                                .spacing([30.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    if self.right_view_selected.is_some() {
                                        match self.right_view_selected.as_ref().unwrap() {
                                            ForeignView::Facilitator => {
                                                let facilitators = if self
                                                    .facilitator_filter
                                                    .is_empty()
                                                {
                                                    self.db.get_all_support_workers()
                                                } else {
                                                    self.db.get_filtered_support_workers(format!(
                                                        "first_name LIKE '%{}%'",
                                                        self.facilitator_filter.clone()
                                                    ))
                                                };
                                                let size = facilitators.len();
                                                ui.label("Facilitator Name");
                                                ui.end_row();
                                                for index in 0..size {
                                                    if ui
                                                        .button(format!(
                                                            "{} {}",
                                                            &facilitators[index].first_name,
                                                            &facilitators[index].last_name
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.facilitator = facilitators[index].clone();
                                                    }
                                                    /*
                                                    ui.label(&workshops[index].first_name.clone());
                                                    ui.label(&workshops[index].first_name.clone());
                                                    ui.label(&workshops[index].facilitator.clone().to_string());
                                                    ui.label(&workshops[index].venue.clone().to_string());
                                                    ui.label(&workshops[index].start_date.clone().to_string());
                                                    ui.label(&workshops[index].end_date.clone().to_string());

                                                    */
                                                    ui.end_row();
                                                }
                                            }
                                            ForeignView::Venue => {
                                                let venues = if self
                                                    .venue_filter
                                                    .is_empty()
                                                {
                                                    self.db.get_all_venues()
                                                } else {
                                                    self.db.get_filtered_venues(format!(
                                                        "name LIKE '%{}%'",
                                                        self.venue_filter.clone()
                                                    ))
                                                };
                                                let size = venues.len();
                                                ui.label("Venue Name");
                                                ui.end_row();
                                                for index in 0..size {
                                                    if ui
                                                        .button(format!(
                                                            "{}",
                                                            &venues[index].name,
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.venue = venues[index].clone();
                                                    }
                                                    ui.end_row();
                                                }
                                            }
                                        }
                                    } else {
                                        ui.label("SELECT AN INPUT");
                                    }
                                });
                        });
                    });
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
                                ui.horizontal(|ui|
                                    {
                                        if self.facilitator.id.is_some() {
                                            ui.label(format!("{} {}", self.facilitator.first_name, self.facilitator.last_name));
                                        }
                                        if ui.add(TextEdit::singleline(&mut self.facilitator_filter).hint_text("facilitator")).has_focus() {
                                            self.right_view_selected = Some(ForeignView::Facilitator);
                                        }
                                    });
                                ui.end_row();
                                ui.label("Venue:");
                                ui.horizontal(|ui| {
                                    if self.venue.id.is_some() {
                                        ui.label(format!("{}", self.venue.name));
                                    }
                                    if ui.add(TextEdit::singleline(&mut self.venue_filter).hint_text("venue")).has_focus() {
                                        self.right_view_selected = Some(ForeignView::Venue);
                                    }});
                                ui.end_row();
                                ui.label("Start Date:");
                                ui.add(
                                    DatePickerButton::new(&mut self.start_date)
                                        .format("%d-%m-%Y")
                                        .highlight_weekends(false)
                                        .id_source("start_date"),
                                );
                                ui.end_row();
                                ui.label("End Date:");
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
                        // todo: need to add data validation.
                        if ui.button("✔ Confirm").clicked() {
                            let edited_workshop = Workshop {
                                id: self.workshop_check.id,
                                name: self.name.clone(),
                                facilitator: self.facilitator.id.unwrap(),
                                venue: self.venue.id.unwrap(),
                                start_date: self.start_date.clone(),
                                end_date: self.end_date.clone(),
                            };
                            self.db.edit_workshop(edited_workshop).unwrap();
                            self.workshop_check.id = None;
                            self.changed = true;
                        };
                        if ui.button("❌ Delete").clicked() {
                            // todo: need to add confirmation button.
                            self.db.delete_workshop(self.workshop_check.id.unwrap());
                            self.workshop_check.id = None;
                            self.changed = true;
                        };
                    });
                }
            });
    }
}
