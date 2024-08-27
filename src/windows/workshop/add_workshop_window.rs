use crate::database_logic::data_structs::{SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;
use egui::{Context, TextEdit};
use egui_extras::DatePickerButton;

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
pub struct AddWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,
    right_view_selected: Option<ForeignView>,

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

impl AddWindow {
    pub fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Create Workshop")
            .open(&mut self.open)
            .max_height(110.0)
            .default_width(500.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::SidePanel::right("create_workshop_right_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            egui::Grid::new("foreign_add_filter_results")
                                .num_columns(6)
                                .spacing([30.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    if self.right_view_selected.is_some() {
                                        match self.right_view_selected.as_ref().unwrap() {
                                            ForeignView::Facilitator => {
                                                let facilitators =
                                                    if self.facilitator_filter.is_empty() {
                                                        self.db.get_all_support_workers(
                                                            String::from(""),
                                                        )
                                                    } else {
                                                        self.db.get_filtered_support_workers(
                                                            format!(
                                                                "first_name LIKE '%{}%'",
                                                                self.facilitator_filter.clone()
                                                            ),
                                                            String::from(""),
                                                        )
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
                                                let venues = if self.venue_filter.is_empty() {
                                                    self.db.get_all_venues(String::from(""))
                                                } else {
                                                    self.db.get_filtered_venues(
                                                        format!(
                                                            "name LIKE '%{}%'",
                                                            self.venue_filter.clone()
                                                        ),
                                                        String::from(""),
                                                    )
                                                };
                                                ui.label("Venue Name");
                                                ui.end_row();
                                                for venue in venues {
                                                    if ui.button(venue.name.to_string()).clicked() {
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
                                                    } else {
                                                        if ui
                                                            .button(format!(
                                                                "{} {}",
                                                                participant.first_name,
                                                                participant.last_name
                                                            ))
                                                            .clicked()
                                                        {
                                                            self.participants.push(id);
                                                        }
                                                    }
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
                                                    } else {
                                                        if ui
                                                            .button(format!(
                                                                "{} {}",
                                                                support_worker.first_name,
                                                                support_worker.last_name
                                                            ))
                                                            .clicked()
                                                        {
                                                            self.support_workers.push(id);
                                                        }
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
                    egui::Grid::new("create_workshop_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name:");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
                            ui.end_row();
                            ui.label("Facilitator:");
                            ui.horizontal(|ui| {
                                if self.facilitator.id.is_some() {
                                    ui.label(format!(
                                        "{} {}",
                                        self.facilitator.first_name, self.facilitator.last_name
                                    ));
                                }
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.facilitator_filter)
                                            .hint_text("facilitator"),
                                    )
                                    .has_focus()
                                {
                                    self.right_view_selected = Some(ForeignView::Facilitator);
                                }
                            });
                            ui.end_row();
                            ui.label("Participants:");
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
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.venue_filter)
                                            .hint_text("venue"),
                                    )
                                    .has_focus()
                                {
                                    self.right_view_selected = Some(ForeignView::Venue);
                                }
                            });
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
                    if ui.button("➕ ADD").clicked() {
                        let new_workshop = Workshop {
                            id: None,
                            name: self.name.clone(),
                            facilitator: self.facilitator.id.unwrap(),
                            venue: self.venue.id.unwrap(),
                            start_date: self.start_date,
                            end_date: self.end_date,
                        };
                        self.db.add_workshop(new_workshop).unwrap();
                        let workshop_id = self.db.get_filtered_workshops(format!("name = '{}' AND facilitator = '{}' AND venue = '{}' AND start_date = '{}' AND end_date = '{}'", &self.name, &self.facilitator.id.unwrap(), &self.venue.id.unwrap(), &self.start_date.to_string(), &self.end_date.to_string()), String::new())[0].id;
                        self.db.add_participants_to_workshop(&self.participants, workshop_id.unwrap()).unwrap();
                        self.db.add_support_workers_to_workshop(&self.support_workers, workshop_id.unwrap()).unwrap();
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
