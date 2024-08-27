use crate::database_logic::data_structs::{Venue, Sort};
use crate::database_logic::database::DataBase;
use crate::windows::venue::{add_venue_window::AddWindow, edit_venue_window::EditWindow, filter_venue_window::FilterWindow};
use egui::{Context, Ui};

//todo: comment code

fn sort_to_string(sort: &Sort) -> String {
    match sort {
        Sort::AlphabeticalAscending => {String::from("ORDER BY name ASC")}
        Sort::AlphabeticalDescending => {String::from("ORDER BY name DESC")}
    }
}

#[derive(Default)]
pub struct VenuesView {
    db: DataBase,

    sort: Sort,
    filter: String,
    add_window: AddWindow,
    edit_window: EditWindow,
    filter_window: FilterWindow,

    selected_venue: Venue,
}

impl VenuesView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let venues = if self.filter.is_empty() {
            self.db.get_all_venues(sort_to_string(&self.sort))
        } else { 
            self.db.get_filtered_venues(self.filter.clone(), sort_to_string(&self.sort))
        };
        egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Address");
                ui.label("Postcode");
                ui.label("State");
                ui.label("Price");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("venue_results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for venue in venues {
                        if ui.button(venue.name.to_string()).clicked() {
                            self.selected_venue = venue.clone();
                        }
                        ui.label(venue.address.clone().unwrap());
                        ui.label(venue.postcode.clone().unwrap());
                        ui.label(venue.state.clone().unwrap());
                        ui.label(venue.price.clone().unwrap());
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
                if self.selected_venue.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(self.selected_venue.name.to_string());
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!("Name: {}", self.selected_venue.name));
                        ui.label(format!(
                            "address: {}",
                            self.selected_venue.address.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "suburb: {}",
                            self.selected_venue.suburb.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "postcode: {}",
                            self.selected_venue.postcode.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "state: {}",
                            self.selected_venue.state.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "description: {}",
                            self.selected_venue.description.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "contact_person_name: {}",
                            self.selected_venue
                                .contact_person_name
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "contact_person_phone: {}",
                            self.selected_venue
                                .contact_person_phone
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "venue_phone_number: {}",
                            self.selected_venue
                                .venue_phone_number
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "price: {}",
                            self.selected_venue.price.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "notes: {}",
                            self.selected_venue.notes.clone().unwrap_or_default()
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
        self.edit_window.ui(ui, ctx, self.selected_venue.clone());
        self.filter = self.filter_window.ui(ui, ctx);
    }
}
