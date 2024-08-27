use std::fmt::{format, Display};
use crate::database_logic::data_structs::{Workshop, Sort, Participant, SupportWorker};
use crate::database_logic::database::DataBase;
use crate::windows::workshop::{
    add_workshop_window::AddWindow, edit_workshop_window::EditWindow,
    filter_workshop_window::FilterWindow,
};
use egui::{Context, Ui};

//todo: comment code

fn sort_to_string(sort: &Sort) -> String {
    match sort {
        Sort::AlphabeticalAscending => {String::from("ORDER BY name ASC")}
        Sort::AlphabeticalDescending => {String::from("ORDER BY name DESC")}
    }
}

#[derive(Default)]
pub struct WorkshopsView {
    db: DataBase,

    sort: Sort,
    filter: String,
    add_window: AddWindow,
    edit_window: EditWindow,
    filter_window: FilterWindow,

    selected_workshop: Workshop,
    participants: Vec<Participant>,
    support_workers: Vec<SupportWorker>,
}

impl WorkshopsView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let workshops = if self.filter.is_empty() {
            self.db.get_all_workshops(sort_to_string(&self.sort))
        } else {
            self.db.get_filtered_workshops(self.filter.clone(), sort_to_string(&self.sort))
        };
        egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Facilitator");
                ui.label("Venue");
                ui.label("Start Date");
                ui.label("End Date");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("venue_results")
                .num_columns(4)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for workshop in workshops {
                        if ui.button(workshop.name.to_string()).clicked() {
                            self.participants.clear();
                            self.support_workers.clear();
                            self.selected_workshop = workshop.clone();
                            for participant in self.db.get_participants_from_workshop(self.selected_workshop.id.unwrap()) {
                                if !self.db.get_filtered_participants(format!("id = '{}'", participant), String::new()).is_empty() {
                                    self.participants.push(self.db.get_filtered_participants(format!("id = '{}'", participant), String::new())[0].clone());
                                }
                            }
                            for support_worker in self.db.get_support_workers_from_workshop(self.selected_workshop.id.unwrap()) {
                                if !self.db.get_filtered_support_workers(format!("id = '{}'", support_worker), String::new()).is_empty() {
                                    self.support_workers.push(self.db.get_filtered_support_workers(format!("id = '{}'", support_worker), String::new())[0].clone());
                                }
                                }
                        }
                        ui.label(format!("{} {}", self.db.get_filtered_support_workers(format!("id = '{}'", workshop.facilitator), String::new())[0].first_name, self.db.get_filtered_support_workers(format!("id = '{}'", workshop.facilitator), String::new())[0].last_name));
                        ui.label(self.db.get_filtered_venues(format!("id = '{}'", workshop.venue), String::new())[0].name.clone());
                        ui.label(workshop.start_date.clone().to_string());
                        ui.label(workshop.end_date.clone().to_string());
                        ui.end_row();
                    }
                });
        });
    }
    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                if self.selected_workshop.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(self.selected_workshop.name.to_string());
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!("Name: {}", self.selected_workshop.name));
                        ui.label(format!("Facilitator: {} {}", self.db.get_filtered_support_workers(format!("id = '{}'", self.selected_workshop.facilitator), String::new())[0].first_name, self.db.get_filtered_support_workers(format!("id = '{}'", self.selected_workshop.facilitator), String::new())[0].last_name));
                        ui.label(format!("Venue: {}", self.db.get_filtered_venues(format!("id = '{}'", self.selected_workshop.venue), String::new())[0].name.clone()));
                        ui.label(format!(
                            "end_date: {}",
                            self.selected_workshop.end_date.clone()
                        ));
                        for participant in self.participants.clone() {
                            ui.label(format!("Participant: {} {}", participant.first_name, participant.last_name));
                        }
                        for support_worker in self.support_workers.clone() {
                            ui.label(format!("Support Worker: {} {}", support_worker.first_name, support_worker.last_name));
                        }

                        ui.label(format!(
                            "start_date: {}",
                            self.selected_workshop.start_date.clone()
                        ));
                        ui.label(format!(
                            "end_date: {}",
                            self.selected_workshop.end_date.clone()
                        ));
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT WORKSHOP");
                    });
                }
            });
    }

    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(25.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("➕ Create").clicked() {
                        self.add_window.open = !self.add_window.open;
                    };
                    if ui.button("✏ Edit").clicked() {
                        self.edit_window.open = !self.edit_window.open;
                    };
                    if ui.button("⛭ Filter").clicked() {
                        self.filter_window.open = !self.filter_window.open;
                    };
                    ui.label("Sort: ");
                    egui::ComboBox::from_label("")
                        .selected_text(match self.sort {
                            Sort::AlphabeticalAscending => {String::from("Ascending")}
                            Sort::AlphabeticalDescending => {String::from("Descending")}
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.sort,
                                Sort::AlphabeticalAscending,
                                "Ascending",
                            );
                            ui.selectable_value(
                                &mut self.sort,
                                Sort::AlphabeticalDescending,
                                "Descending",
                            );
                        });
                });
            });
    }
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ctx);
        self.edit_window.ui(ui, ctx, self.selected_workshop.clone());
        self.filter = self.filter_window.ui(ui, ctx);
    }
}
