use crate::database_logic::data_structs::{Sort, SupportWorker}; // Importing the Sort and SupportWorker structs from the data_structs module in database_logic.
use crate::database_logic::database::DataBase; // Importing the DataBase struct from the database module in database_logic.
use crate::windows::support_worker::{ // Importing the AddWindow, EditWindow, and FilterWindow structs from the support_worker module in the window's module.
                                      add_support_worker_window::AddWindow,
                                      edit_support_worker_window::EditWindow,
                                      filter_support_worker_window::FilterWindow
};
use egui::{Context, Ui}; // Importing the Context and Ui structs from the egui crate.

// Function to convert a Sort enum variant to a corresponding SQL string.
fn sort_to_string(sort: &Sort) -> String {
    match sort {
        Sort::AlphabeticalAscending => {String::from("ORDER BY first_name ASC")} // Return SQL for ascending order by first name.
        Sort::AlphabeticalDescending => {String::from("ORDER BY first_name DESC")} // Return SQL for descending order by first name.
    }
}

#[derive(Default)]
pub struct SupportWorkersView { // Defining a struct to represent the view for support workers.
    db: DataBase, // Database instance for managing support workers data.
    filter: String, // Filter string to apply to the support workers.
    sort: Sort, // Sort option for ordering the support workers.
    add_window: AddWindow, // Instance of the add support worker window.
    edit_window: EditWindow, // Instance of the edit support worker window.
    filter_window: FilterWindow, // Instance of the filter support worker window.
    selected_support_worker: SupportWorker, // Currently selected support worker.
}

impl SupportWorkersView {
    // Function to render the main UI.
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui); // Render the right panel view.
        self.bottom_menu_view(ui); // Render the bottom menu view.
        self.load_windows_ui(ui, ctx); // Load the UI for the different windows.
        self.main_view(ui); // Render the main view.
    }

    // Function to render the main view with the list of support workers.
    fn main_view(&mut self, ui: &mut Ui) {
        let support_workers = if self.filter.is_empty() { // Check if the filter is empty.
            self.db.get_all_support_workers(sort_to_string(&self.sort)) // Get all support workers sorted if no filter is applied.
        } else {
            self.db.get_filtered_support_workers(self.filter.clone(), sort_to_string(&self.sort)) // Get filtered support workers based on the filter and sort.
        };
        egui::Grid::new("headings") // Create a grid for the table headings.
            .num_columns(5) // Set the number of columns.
            .spacing([30.0, 4.0]) // Set spacing between columns and rows.
            .striped(false) // Disable striped rows.
            .show(ui, |ui| { // Display the grid with the headings.
                ui.label("Name");
                ui.label("Date of Birth");
                ui.label("Phone Number");
                ui.label("Email");
                ui.label("Car Insurance");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| { // Create a scrollable area for the support workers list.
            egui::Grid::new("support_worker_results") // Create a grid for displaying support workers.
                .num_columns(5) // Set the number of columns.
                .spacing([30.0, 4.0]) // Set spacing between columns and rows.
                .striped(true) // Enable striped rows.
                .show(ui, |ui| { // Display the grid with support workers data.
                    for support_worker in support_workers { // Iterate over the list of support workers.
                        if ui
                            .button(format!(
                                "{} {}",
                                support_worker.first_name,
                                support_worker.last_name
                            ))
                            .clicked()
                        {
                            self.selected_support_worker = support_worker.clone(); // Update the selected support worker when clicked.
                        }
                        ui.label(match support_worker.dob { // Display the date of birth if available.
                            None => String::new(),
                            Some(value) => value.format("%d-%m-%Y").to_string(),
                        });
                        ui.label(support_worker.phone); // Display the phone number.
                        ui.label(support_worker.email); // Display the email address.
                        ui.label(match support_worker.car_insurance { // Display car insurance info if available.
                            None => String::new(),
                            Some(value) => value.to_string(),
                        });
                        ui.end_row();
                    }
                });
        });
    }

    // Function to render the right panel with details of the selected support worker.
    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel") // Create a right-side panel.
            .resizable(true) // Allow resizing of the panel.
            .default_width(150.0) // Set the default width of the panel.
            .show_inside(ui, |ui| { // Display the panel contents.
                if self.selected_support_worker.id.is_some() { // Check if a support worker is selected.
                    ui.vertical_centered(|ui| {
                        ui.heading(format!(
                            "{} {}",
                            self.selected_support_worker.first_name,
                            self.selected_support_worker.last_name
                        ));
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| { // Create a scrollable area for support worker details.
                        ui.label(format!(
                            "Name: {} {}",
                            self.selected_support_worker.first_name,
                            self.selected_support_worker.last_name
                        ));
                        ui.label(format!(
                            "dob: {}",
                            self.selected_support_worker.dob.unwrap_or_default()
                        ));
                        ui.label(format!("phone: {}", self.selected_support_worker.phone)); // Display phone number.
                        ui.label(format!("email: {}", self.selected_support_worker.email)); // Display email address.
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
                    ui.vertical_centered(|ui| { // Display message when no support worker is selected.
                        ui.heading("SELECT SUPPORT WORKER");
                    });
                }
            });
    }

    // Function to render the bottom menu with options like Create, Edit, Filter, and Reset DB.
    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel") // Create a bottom panel.
            .resizable(false) // Disable resizing of the panel.
            .max_height(25.0) // Set the maximum height of the panel.
            .show_inside(ui, |ui| { // Display the panel contents.
                ui.horizontal(|ui| { // Create a horizontal layout for the buttons.
                    if ui.button("➕ Create").clicked() { // Button to open the AddWindow.
                        self.add_window.open = !self.add_window.open;
                    };
                    if ui.button("✏ Edit").clicked() { // Button to open the EditWindow.
                        self.edit_window.open = !self.edit_window.open;
                    };
                    if ui.button("⛭ Filter").clicked() { // Button to open the FilterWindow.
                        self.filter_window.open = !self.filter_window.open;
                    };
                    ui.label("Sort: "); // Label for the sort options.
                    egui::ComboBox::from_label("")
                        .selected_text(match self.sort { // Display the current sort option.
                            Sort::AlphabeticalAscending => {String::from("Ascending")}
                            Sort::AlphabeticalDescending => {String::from("Descending")}
                        })
                        .show_ui(ui, |ui| { // Create the dropdown for selecting sort options.
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

    // Function to load the UI elements for the Add, Edit, and Filter windows.
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ui, ctx); // Load the AddWindow UI.
        self.edit_window
            .ui(ui, ctx, self.selected_support_worker.clone()); // Load the EditWindow UI with the selected support worker.
        self.filter = self.filter_window.ui(ui, ctx); // Load the FilterWindow UI and update the filter string.
    }
}
