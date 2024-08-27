// This code defines an `EditWindow` structure with various fields related to editing workshop information,
// such as the workshop's name, facilitator, venue, participants, and support workers.

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
    Participants,
    SupportWorkers
}

#[derive(Default)]
pub struct EditWindow {
    pub open: bool, // Indicates whether the edit window is open.
    pub db: DataBase, // The database instance for performing data operations.

    right_view_selected: Option<ForeignView>, // Stores the currently selected view for filtering.
    changed: bool, // Tracks whether changes have been made.
    workshop_check: Workshop, // A copy of the workshop being edited to track changes.

    // Fields for storing various workshop-related data.
    name: String,
    support_workers: Vec<i32>,
    support_worker_filter: String,
    participants: Vec<i32>,
    participant_filter: String,
    facilitator: SupportWorker,
    facilitator_filter: String,
    venue: Venue,
    venue_filter: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl EditWindow {
    // The `ui` function handles the rendering of the edit window's user interface.
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context, workshop: Workshop) {
        // If the workshop ID has changed, update the fields with the new workshop's data.
        if self.workshop_check.id != workshop.id {
            self.workshop_check = workshop.clone();
            self.facilitator = self.db.get_filtered_support_workers(format!("id = '{}'", workshop.facilitator), String::from(""))[0].clone();
            self.venue = self.db.get_filtered_venues(format!("id = '{}'", workshop.venue), String::from(""))[0].clone();
            self.participants = self.db.get_participants_from_workshop(self.workshop_check.id.unwrap());
            self.support_workers = self.db.get_support_workers_from_workshop(self.workshop_check.id.unwrap());
            self.name = workshop.name;
            self.start_date = workshop.start_date;
            self.end_date = workshop.end_date;
        }
        // If the edit window is not open, reset the `changed` flag.
        if !self.open {
            self.changed = false;
        };
        // Render the main edit window.
        egui::Window::new("Edit Workshop")
            .open(&mut self.open)
            .max_height(110.0)
            .default_width(500.0)
            .show(ctx, |ui| {
                // Render the right-side panel for selecting and filtering foreign data.
                egui::SidePanel::right("edit_workshop_right_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("foreign_edit_filter_results")
                                .num_columns(6)
                                .spacing([30.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    // Depending on the selected view, display the relevant data and filters.
                                    if self.right_view_selected.is_some() {
                                        match self.right_view_selected.as_ref().unwrap() {
                                            ForeignView::Facilitator => {
                                                let facilitators = if self
                                                    .facilitator_filter
                                                    .is_empty()
                                                {
                                                    self.db.get_all_support_workers(String::from(""))
                                                } else {
                                                    self.db.get_filtered_support_workers(format!(
                                                        "first_name LIKE '%{}%'",
                                                        self.facilitator_filter.clone()
                                                    ), String::from(""))
                                                };
                                                ui.label("Facilitator Name");
                                                ui.end_row();
                                                for facilitator in facilitators {
                                                    if ui
                                                        .button(format!(
                                                            "{} {}",
                                                            facilitator.first_name,
                                                            facilitator.last_name
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.facilitator = facilitator.clone();
                                                    }
                                                    /*
                                                    // These commented-out lines would display additional details.
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
                                                    self.db.get_all_venues(String::from(""))
                                                } else {
                                                    self.db.get_filtered_venues(format!(
                                                        "name LIKE '%{}%'",
                                                        self.venue_filter.clone()
                                                    ), String::from(""))
                                                };
                                                ui.label("Venue Name");
                                                ui.end_row();
                                                for venue in venues {
                                                    if ui
                                                        .button(venue.name.to_string())
                                                        .clicked()
                                                    {
                                                        self.venue = venue.clone();
                                                    }
                                                    ui.end_row();
                                                }
                                            }
                                            ForeignView::Participants => {
                                                let participants =
                                                    if self.participant_filter.is_empty() {
                                                        self.db
                                                            .get_all_participants(String::from(""))
                                                    } else {
                                                        self.db.get_filtered_participants(
                                                            format!(
                                                                "first_name LIKE '%{}%'",
                                                                self.participant_filter.clone()
                                                            ),
                                                            String::from(""),
                                                        )
                                                    };
                                                ui.label("Participant Name");
                                                ui.end_row();
                                                for participant in participants {
                                                    let id = participant.id.unwrap();
                                                    if self.participants.contains(&id) {
                                                        if ui
                                                            .button(format!(
                                                                "☑ {} {}",
                                                                participant.first_name,
                                                                participant.last_name
                                                            ))
                                                            .clicked()
                                                        {
                                                            self.participants
                                                                .retain(|&value| value != id);
                                                        }
                                                    } else if ui
                                                        .button(format!(
                                                            "{} {}",
                                                            participant.first_name,
                                                            participant.last_name
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.participants.push(id);
                                                    }
                                                    /*
                                                    // These commented-out lines would display additional details.
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
                                            ForeignView::SupportWorkers => {
                                                let support_workers =
                                                    if self.support_worker_filter.is_empty() {
                                                        self.db
                                                            .get_all_support_workers(String::from(""))
                                                    } else {
                                                        self.db.get_filtered_support_workers(
                                                            format!(
                                                                "first_name LIKE '%{}%'",
                                                                self.support_worker_filter.clone()
                                                            ),
                                                            String::from(""),
                                                        )
                                                    };
                                                ui.label("Support Worker Name");
                                                ui.end_row();
                                                for support_worker in support_workers {
                                                    let id = support_worker.id.unwrap();
                                                    if self.support_workers.contains(&id) {
                                                        if ui
                                                            .button(format!(
                                                                "☑ {} {}",
                                                                support_worker.first_name,
                                                                support_worker.last_name
                                                            ))
                                                            .clicked()
                                                        {
                                                            self.support_workers
                                                                .retain(|&value| value != id);
                                                        }
                                                    } else if ui
                                                        .button(format!(
                                                            "{} {}",
                                                            support_worker.first_name,
                                                            support_worker.last_name
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.support_workers.push(id);
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
                // Display a message if changes were made or if a workshop needs to be selected.
                if self.changed {
                    ui.label("DONE ✔");
                    ui.label("Reselect Workshop to see changes.");
                } else if self.workshop_check.id.is_none() {
                    ui.label("Select a Workshop to EDIT");
                } else {
                    // Render the main form for editing workshop details.
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("edit_workshop_grid")
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
                                ui.label("Participants: ");
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.participant_filter)
                                            .hint_text("participant"),
                                    )
                                    .has_focus()
                                {
                                    self.right_view_selected = Some(ForeignView::Participants);
                                }
                                ui.end_row();
                                ui.label("Support Workers:");
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.support_worker_filter)
                                            .hint_text("participant"),
                                    )
                                    .has_focus()
                                {
                                    self.right_view_selected = Some(ForeignView::SupportWorkers);
                                }
                                ui.end_row();
                                ui.label("Venue:");
                                ui.horizontal(|ui| {
                                    if self.venue.id.is_some() {
                                        ui.label(self.venue.name.to_string());
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
                    // Render the Confirm and Delete buttons.
                    ui.horizontal(|ui| {
                        if ui.button("✔ Confirm").clicked() {
                            let edited_workshop = Workshop {
                                id: self.workshop_check.id,
                                name: self.name.clone(),
                                facilitator: self.facilitator.id.unwrap(),
                                venue: self.venue.id.unwrap(),
                                start_date: self.start_date,
                                end_date: self.end_date,
                            };
                            self.db.edit_workshop(edited_workshop).unwrap();
                            self.db.delete_workshop_participants(self.workshop_check.id.unwrap()).unwrap();
                            self.db.add_participants_to_workshop(&self.participants, self.workshop_check.id.unwrap()).unwrap();
                            self.db.delete_workshop_support_workers(self.workshop_check.id.unwrap()).unwrap();
                            self.db.add_support_workers_to_workshop(&self.support_workers, self.workshop_check.id.unwrap()).unwrap();
                            self.workshop_check.id = None;
                            self.changed = true;
                        };
                        if ui.button("❌ Delete").clicked() {
                            // todo: need to add confirmation button.
                            self.db.delete_workshop(self.workshop_check.id.unwrap()).unwrap();
                            self.workshop_check.id = None;
                            self.changed = true;
                        };
                    });
                }
            });
    }
}
