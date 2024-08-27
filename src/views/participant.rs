// Importing necessary modules and traits
use crate::database_logic::data_structs::{Participant, Sort};
use crate::database_logic::database::DataBase;
use crate::windows::participant::{
    add_participant_window::AddWindow, edit_participant_window::EditWindow,
    filter_participant_window::FilterWindow
};
use egui::{Context, Ui};

// Function to convert Sort enum to corresponding SQL order by clause
fn sort_to_string(sort: &Sort) -> String {
    match sort {
        // If the sort is AlphabeticalAscending, return SQL clause for ascending order
        Sort::AlphabeticalAscending => {String::from("ORDER BY first_name ASC")}
        // If the sort is AlphabeticalDescending, return SQL clause for descending order
        Sort::AlphabeticalDescending => {String::from("ORDER BY first_name DESC")}
    }
}

// Struct representing the ParticipantsView, which manages the UI for participant data
#[derive(Default)]
pub struct ParticipantsView {
    db: DataBase,                  // Database instance to interact with participant data
    sort: Sort,                    // Current sort order
    filter: String,                // Current filter applied to participant data
    add_window: AddWindow,         // Instance of the window for adding a participant
    edit_window: EditWindow,       // Instance of the window for editing a participant
    filter_window: FilterWindow,   // Instance of the window for filtering participants
    selected_participant: Participant, // The currently selected participant
}

impl ParticipantsView {
    // Method to build and display the main UI elements
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);    // Display the right panel view
        self.bottom_menu_view(ui);    // Display the bottom menu view
        self.load_windows_ui(ui, ctx); // Load and display the windows UI
        self.main_view(ui);           // Display the main view with participant data
    }

    // Method to display the main participant data view
    fn main_view(&mut self, ui: &mut Ui) {
        // Get the list of participants based on the current filter and sort settings
        let participants = if self.filter.is_empty() {
            self.db.get_all_participants(sort_to_string(&self.sort)) // Get all participants if no filter
        } else {
            self.db.get_filtered_participants(self.filter.clone(), sort_to_string(&self.sort)) // Get filtered participants
        };
        // Create a grid to display the column headings
        egui::Grid::new("headings")
            .num_columns(5) // Number of columns
            .spacing([30.0, 4.0]) // Spacing between columns
            .striped(false) // Disable row striping
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Date of Birth");
                ui.label("Phone Number");
                ui.label("Email");
                ui.label("Support Ratio");
                ui.end_row(); // End the heading row
            });
        // Create a scrollable area to display participant data
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("participant_results")
                .num_columns(5) // Number of columns
                .spacing([30.0, 4.0]) // Spacing between columns
                .striped(true) // Enable row striping for better readability
                .show(ui, |ui| {
                    // Iterate through each participant and display their data
                    for participant in participants {
                        // Display a button with the participant's name, and set the selected participant if clicked
                        if ui
                            .button(format!(
                                "{} {}",
                                participant.first_name, participant.last_name
                            ))
                            .clicked()
                        {
                            self.selected_participant = participant.clone();
                        }
                        // Display participant's date of birth, formatted or empty if none
                        ui.label(match participant.dob {
                            None => String::new(),
                            Some(value) => value.format("%d-%m-%Y").to_string(),
                        });
                        // Display participant's phone number or empty if none
                        ui.label(match participant.phone.clone() {
                            None => String::new(),
                            Some(value) => value.to_string(),
                        });
                        // Display participant's email or empty if none
                        ui.label(match participant.email.clone() {
                            None => String::new(),
                            Some(value) => value.to_string(),
                        });
                        // Display participant's support ratio or empty if none
                        ui.label(match participant.support_ratio {
                            None => String::new(),
                            Some(value) => {
                                format!("1:{}", value)
                            }
                        });
                        ui.end_row(); // End the row for this participant
                    }
                });
        });
    }

    // Method to display the right panel with detailed participant information
    fn right_panel_view(&mut self, ui: &mut Ui) {
        // Create a resizable side panel on the right
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                // If a participant is selected, display their detailed information
                if self.selected_participant.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(format!(
                            "{} {}",
                            self.selected_participant.first_name,
                            self.selected_participant.last_name
                        ));
                    });
                    // Scrollable area to display all detailed fields of the participant
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!(
                            "Name: {} {}",
                            self.selected_participant.first_name,
                            self.selected_participant.last_name
                        ));
                        ui.label(format!(
                            "medicare_number: {}",
                            self.selected_participant.medicare_number
                        ));
                        ui.label(format!(
                            "dob: {}",
                            self.selected_participant.dob.unwrap_or_default()
                        ));
                        ui.label(format!(
                            "address: {}",
                            self.selected_participant
                                .address
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "suburb: {}",
                            self.selected_participant.suburb.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "postcode: {}",
                            self.selected_participant
                                .postcode
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "phone: {}",
                            self.selected_participant.phone.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "email: {}",
                            self.selected_participant.email.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "medical_notes: {}",
                            self.selected_participant
                                .medical_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "dietary_notes: {}",
                            self.selected_participant
                                .dietary_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "physical_notes: {}",
                            self.selected_participant
                                .physical_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "other_notes: {}",
                            self.selected_participant
                                .other_notes
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "support_ratio: {}",
                            self.selected_participant
                                .support_ratio
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "photo_permission: {}",
                            self.selected_participant.photo_permission.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "private_hospital_preference: {}",
                            self.selected_participant
                                .private_hospital_preference
                                .unwrap_or(false)
                        ));
                        ui.label(format!(
                            "private_health_insurancer: {}",
                            self.selected_participant
                                .private_health_insurer
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "private_health_number: {}",
                            self.selected_participant
                                .private_health_number
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "communication_preference: {}",
                            self.selected_participant
                                .communication_preference
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "ndis_plan_number: {}",
                            self.selected_participant
                                .ndis_plan_number
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "ndis_plan_start_date: {}",
                            self.selected_participant
                                .ndis_plan_start_date
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "core_funding: {}",
                            self.selected_participant.core_funding.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "capacity_building_funding: {}",
                            self.selected_participant
                                .capacity_building_funding
                                .unwrap_or(false)
                        ));
                        ui.label(format!(
                            "self_managed: {}",
                            self.selected_participant.self_managed.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "plan_managed: {}",
                            self.selected_participant.plan_managed.unwrap_or(false)
                        ));
                        ui.label(format!(
                            "ndis_plan_end_date: {}",
                            self.selected_participant
                                .ndis_plan_end_date
                                .unwrap_or_default()
                        ));
                    });
                } else {
                    // If no participant is selected, display a prompt to select one
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT PARTICIPANT");
                    });
                }
            });
    }

    // Method to display the bottom menu with options to create, edit, and filter participants
    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        // Create a top-bottom panel at the bottom of the screen
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(25.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    // Button to toggle the add participant window
                    if ui.button("➕ Create").clicked() {
                        self.add_window.open = !self.add_window.open;
                    };
                    // Button to toggle the edit participant window
                    if ui.button("✏ Edit").clicked() {
                        self.edit_window.open = !self.edit_window.open;
                    };
                    // Button to toggle the filter participant window
                    if ui.button("⛭ Filter").clicked() {
                        self.filter_window.open = !self.filter_window.open;
                    };
                    ui.label("Sort: "); // Label for the sort dropdown
                    // ComboBox to select the sorting order
                    egui::ComboBox::from_label("")
                        .selected_text(match self.sort {
                            Sort::AlphabeticalAscending => {String::from("Ascending")}
                            Sort::AlphabeticalDescending => {String::from("Descending")}
                        })
                        .show_ui(ui, |ui| {
                            // Option to select ascending sort order
                            ui.selectable_value(
                                &mut self.sort,
                                Sort::AlphabeticalAscending,
                                "Ascending",
                            );
                            // Option to select descending sort order
                            ui.selectable_value(
                                &mut self.sort,
                                Sort::AlphabeticalDescending,
                                "Descending",
                            );
                        });
                });
            });
    }

    // Method to load and display the add, edit, and filter participant windows
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ui, ctx); // Load and display the add participant window
        self.edit_window.ui(ui, ctx, self.selected_participant.clone()); // Load and display the edit participant window
        self.filter = self.filter_window.ui(ui, ctx); // Load and display the filter participant window and update the filter string
    }
}
