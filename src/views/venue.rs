// Importing necessary modules and structs from various parts of the project.
use crate::database_logic::data_structs::{Venue, Sort};
use crate::database_logic::database::DataBase;
use crate::windows::venue::{add_venue_window::AddWindow, edit_venue_window::EditWindow, filter_venue_window::FilterWindow};
use egui::{Context, Ui};

// Converts a Sort enum variant into a corresponding SQL string for ordering query results.
fn sort_to_string(sort: &Sort) -> String {
    match sort {
        Sort::AlphabeticalAscending => {String::from("ORDER BY name ASC")}
        Sort::AlphabeticalDescending => {String::from("ORDER BY name DESC")}
    }
}

// The VenuesView struct holds the state and logic necessary for displaying and interacting with the list of venues.
#[derive(Default)]
pub struct VenuesView {
    db: DataBase, // The database instance for interacting with venue data.

    sort: Sort, // The current sorting order for venues.
    filter: String, // A string used to filter the list of venues.
    add_window: AddWindow, // State for the window that handles adding new venues.
    edit_window: EditWindow, // State for the window that handles editing venues.
    filter_window: FilterWindow, // State for the window that handles filtering venues.

    selected_venue: Venue, // The currently selected venue.
}

impl VenuesView {
    // The main user interface (UI) function that orchestrates the entire view.
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui); // Render the right panel view.
        self.bottom_menu_view(ui); // Render the bottom menu.
        self.load_windows_ui(ui, ctx); // Load any additional windows like add, edit, or filter.
        self.main_view(ui); // Render the main view of venues.
    }

    // The main view that displays a list of venues.
    fn main_view(&mut self, ui: &mut Ui) {
        let venues = if self.filter.is_empty() {
            self.db.get_all_venues(sort_to_string(&self.sort)) // Get all venues if no filter is applied.
        } else {
            self.db.get_filtered_venues(self.filter.clone(), sort_to_string(&self.sort)) // Get filtered venues based on the filter string.
        };

        // Display the headings for the venue list in a grid format.
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

        // Display the list of venues in a scrollable area.
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("venue_results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for venue in venues {
                        // If a venue's button is clicked, set it as the selected venue.
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

    // The right panel view that displays details about the selected venue.
    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                if self.selected_venue.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(self.selected_venue.name.to_string()); // Display the name of the selected venue.
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // Display various details about the selected venue.
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
                        ui.heading("SELECT SUPPORT WORKER"); // Display a message if no venue is selected.
                    });
                }
            });
    }

    // The bottom menu view that provides actions like creating, editing, filtering, and resetting the database.
    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(25.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("➕ Create").clicked() {
                        self.add_window.open = !self.add_window.open; // Toggle the add window.
                    };
                    if ui.button("✏ Edit").clicked() {
                        self.edit_window.open = !self.edit_window.open; // Toggle the edit window.
                    };
                    if ui.button("⛭ Filter").clicked() {
                        self.filter_window.open = !self.filter_window.open; // Toggle the filter window.
                    };
                    if ui.button("RESET DB").clicked() {
                        self.db.drop_db().unwrap(); // Drop the database.
                        self.db.create_db().unwrap(); // Recreate the database.
                    };
                    ui.label("Sort: ");
                    // Display a combo box for selecting the sorting order.
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

    // Loads and displays any additional windows such as the add, edit, and filter windows.
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ui, ctx); // Display the add window UI.
        self.edit_window.ui(ui, ctx, self.selected_venue.clone()); // Display the edit window UI.
        self.filter = self.filter_window.ui(ui, ctx); // Display the filter window UI and get the filter string.
    }
}
