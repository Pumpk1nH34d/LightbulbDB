use std::fmt::{Display};
use crate::database_logic::data_structs::{Sort, SupportWorker};
use crate::database_logic::database::DataBase;
use crate::windows::support_worker::{
    add_support_worker_window::AddWindow, edit_support_worker_window::EditWindow,
    filter_support_worker_window::FilterWindow
};
use egui::{Context, Ui};

//todo: comment code

fn sort_to_string(sort: &Sort) -> String {
    match sort {
        Sort::AlphabeticalAscending => {String::from("ORDER BY first_name ASC")}
        Sort::AlphabeticalDescending => {String::from("ORDER BY first_name DESC")}
    }
}

#[derive(Default)]
pub struct SupportWorkersView {
    db: DataBase,
    filter: String,
    sort: Sort,
    add_window: AddWindow,
    edit_window: EditWindow,
    filter_window: FilterWindow,
    selected_support_worker: SupportWorker,
}

impl SupportWorkersView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let support_workers = if self.filter.is_empty() {
            self.db.get_all_support_workers(sort_to_string(&self.sort))
        } else {
            self.db.get_filtered_support_workers(self.filter.clone(), sort_to_string(&self.sort))
        };
        egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Date of Birth");
                ui.label("Phone Number");
                ui.label("Email");
                ui.label("Car Insurance");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("support_worker_results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for support_worker in support_workers {
                        if ui
                            .button(format!(
                                "{} {}",
                                support_worker.first_name,
                                support_worker.last_name
                            ))
                            .clicked()
                        {
                            self.selected_support_worker = support_worker.clone();
                        }
                        ui.label(match support_worker.dob {
                            None => String::new(),
                            Some(value) => value.format("%d-%m-%Y").to_string(),
                        });
                        ui.label(support_worker.phone);
                        ui.label(support_worker.email);
                        ui.label(match support_worker.car_insurance {
                            None => String::new(),
                            Some(value) => value.to_string(),
                        });
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
                if self.selected_support_worker.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(format!(
                            "{} {}",
                            self.selected_support_worker.first_name,
                            self.selected_support_worker.last_name
                        ));
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!(
                            "Name: {} {}",
                            self.selected_support_worker.first_name,
                            self.selected_support_worker.last_name
                        ));
                        ui.label(format!(
                            "dob: {}",
                            self.selected_support_worker.dob.unwrap_or_default()
                        ));
                        ui.label(format!("phone: {}", self.selected_support_worker.phone));
                        ui.label(format!("email: {}", self.selected_support_worker.email));
                        ui.label(format!(
                            "address: {}",
                            self.selected_support_worker
                                .address
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "suburb: {}",
                            self.selected_support_worker
                                .suburb
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "postcode: {}",
                            self.selected_support_worker
                                .postcode
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "first_aid: {}",
                            self.selected_support_worker.first_aid.unwrap_or_default()
                        ));
                        ui.label(format!(
                            "confidentiality_agreement: {}",
                            self.selected_support_worker
                                .confidentiality_agreement
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "police_clearance: {}",
                            self.selected_support_worker
                                .police_clearance
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "car_insurance: {}",
                            self.selected_support_worker
                                .car_insurance
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "other_qualifications: {}",
                            self.selected_support_worker
                                .other_qualifications
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "notes: {}",
                            self.selected_support_worker
                                .notes
                                .clone()
                                .unwrap_or_default()
                        ));
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT SUPPORT WORKER");
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
                    if ui.button("RESET DB").clicked() {
                        self.db.drop_db().unwrap();
                        self.db.create_db().unwrap();
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
        self.add_window.ui(ui, ctx);
        self.edit_window
            .ui(ui, ctx, self.selected_support_worker.clone());
        self.filter = self.filter_window.ui(ui, ctx);
    }
}
