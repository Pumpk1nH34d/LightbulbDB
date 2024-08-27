use crate::database_logic::data_structs::{Participant, SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};
//todo: comment code

#[derive(Default)]
pub enum ForeignView {
    #[default]
    Facilitator,
    Venue,
    Participants,
    SupportWorkers
}

#[derive(Default)]
pub struct FilterWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    right_view_selected: Option<ForeignView>,
    changed: bool,
    workshop_check: Workshop,

    filter: String,


    name: String,
    facilitator: SupportWorker,
    facilitator_filter: String,
    participant: Participant,
    participant_filter: String,
    support_worker: SupportWorker,
    support_worker_filter: String,
    venue: Venue,
    venue_filter: String,
}

impl FilterWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) -> String {
        if !self.open {
            self.changed = false;
        };
        egui::Window::new("Filter Workshop")
            .open(&mut self.open)
            .max_height(110.0)
            .default_width(500.0)
            .show(ctx, |ui| {
                egui::SidePanel::right("filter_workshop_right_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("foreign_filter_filter_results")
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
                                                let participants = if self
                                                    .participant_filter
                                                    .is_empty()
                                                {
                                                    self.db.get_all_participants(String::from(""))
                                                } else {
                                                    self.db.get_filtered_participants(format!(
                                                        "first_name LIKE '%{}%'",
                                                        self.participant_filter.clone()
                                                    ), String::from(""))
                                                };
                                                ui.label("Participant Name");
                                                ui.end_row();
                                                for participant in participants {
                                                    if ui
                                                        .button(format!(
                                                            "{} {}",
                                                            participant.first_name,
                                                            participant.last_name
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.participant = participant.clone();
                                                    }
                                                    ui.end_row();
                                                }
                                            }
                                            ForeignView::SupportWorkers => {
                                                let support_workers = if self
                                                    .support_worker_filter
                                                    .is_empty()
                                                {
                                                    self.db.get_all_support_workers(String::from(""))
                                                } else {
                                                    self.db.get_filtered_support_workers(format!(
                                                        "first_name LIKE '%{}%'",
                                                        self.support_worker_filter.clone()
                                                    ), String::from(""))
                                                };
                                                ui.label("Participant Name");
                                                ui.end_row();
                                                for support_worker in support_workers {
                                                    if ui
                                                        .button(format!(
                                                            "{} {}",
                                                            support_worker.first_name,
                                                            support_worker.last_name
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.support_worker = support_worker.clone();
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
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("filter_workshop")
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
                                        ui.label(self.venue.name.to_string());
                                    }
                                    if ui.add(TextEdit::singleline(&mut self.venue_filter).hint_text("venue")).has_focus() {
                                        self.right_view_selected = Some(ForeignView::Venue);
                                    }});
                                ui.end_row();
                                ui.label("Participant:");
                                ui.horizontal(|ui|
                                    {
                                        if self.participant.id.is_some() {
                                            ui.label(format!("{} {}", self.participant.first_name, self.participant.last_name));
                                        }
                                        if ui.add(TextEdit::singleline(&mut self.participant_filter).hint_text("participant")).has_focus() {
                                            self.right_view_selected = Some(ForeignView::Participants);
                                        }
                                    });
                                ui.end_row();
                                ui.label("Support Worker:");
                                ui.horizontal(|ui|
                                    {
                                        if self.support_worker.id.is_some() {
                                            ui.label(format!("{} {}", self.support_worker.first_name, self.support_worker.last_name));
                                        }
                                        if ui.add(TextEdit::singleline(&mut self.support_worker_filter).hint_text("support Worker")).has_focus() {
                                            self.right_view_selected = Some(ForeignView::SupportWorkers);
                                        }
                                    });
                                ui.end_row();
                            });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✔ APPLY").clicked() {
                            let mut filter = String::new();
                            if !self.name.is_empty() {
                                filter += &format!("name = '{}' AND ", self.name)
                            };
                            if self.facilitator.id.is_some() {
                                filter += &format!("facilitator = '{}' AND ", self.name)
                            }
                            if self.venue.id.is_some() {
                                filter += &format!("venue = '{}' AND ", self.name)
                            }

                            if !filter.is_empty() {
                                filter.truncate(filter.len() - 4)
                            }
                            self.filter = filter
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
