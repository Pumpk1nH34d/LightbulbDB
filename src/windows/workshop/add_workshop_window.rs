use std::fmt::format;
use crate::database_logic::data_structs::{SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

#[derive(Default)]
enum ForeignView {
    #[default]
    Facilitator,
    Venue,
}

#[derive(Default)]
pub struct AddWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,
    right_view_selected: Option<ForeignView>,

    name: String,
    facilitator: SupportWorker,
    facilitator_filter: String,
    venue: Venue,
    venue_filter: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl AddWindow {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Create Workshop")
            .open(&mut self.open)
            .max_height(110.0)
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
                                                let facilitators = if self
                                                    .facilitator_filter
                                                    .is_empty()
                                                {
                                                    self.db.get_all_support_workers()
                                                } else {
                                                    self.db.get_filtered_support_workers(format!(
                                                        "first_name = '{}'",
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
                                                        "name = '{}'",
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
