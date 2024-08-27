use crate::database_logic::data_structs::{Workshop, Sort, Participant, SupportWorker};
use crate::database_logic::database::DataBase;
use crate::windows::workshop::{
    add_workshop_window::AddWindow, edit_workshop_window::EditWindow,
    filter_workshop_window::FilterWindow,
};
use egui::{Context, Ui};

// The `sort_to_string` function converts the `Sort` enum into an SQL ORDER BY string,
// allowing for sorting of workshop data in ascending or descending alphabetical order.
fn sort_to_string(sort: &Sort) -> String {
    match sort {
        Sort::AlphabeticalAscending => {String::from("ORDER BY name ASC")} // Ascending order by name
        Sort::AlphabeticalDescending => {String::from("ORDER BY name DESC")} // Descending order by name
    }
}

// The `WorkshopsView` struct is the main structure that manages the display and interaction
// with workshops. It includes sorting, filtering, and handling of windows for adding,
// editing, and filtering workshops.
#[derive(Default)]
pub struct WorkshopsView {
    db: DataBase, // Database instance

    sort: Sort, // Current sort order
    filter: String, // Current filter string
    add_window: AddWindow, // Add workshop window
    edit_window: EditWindow, // Edit workshop window
    filter_window: FilterWindow, // Filter workshop window

    selected_workshop: Workshop, // Currently selected workshop
    participants: Vec<Participant>, // List of participants for the selected workshop
    support_workers: Vec<SupportWorker>, // List of support workers for the selected workshop
}

impl WorkshopsView {
    // The `ui` method is the main entry point for rendering the user interface,
    // calling various sub-methods to render different parts of the UI.
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui); // Display the right panel view
        self.bottom_menu_view(ui); // Display the bottom menu view
        self.load_windows_ui(ui, ctx); // Load the UI for the windows
        self.main_view(ui); // Display the main view of workshops
    }

    // The `main_view` method displays the list of workshops, allowing for sorting and filtering.
    fn main_view(&mut self, ui: &mut Ui) {
        let workshops = if self.filter.is_empty() {
            self.db.get_all_workshops(sort_to_string(&self.sort)) // Get all workshops if no filter is applied
        } else {
            self.db.get_filtered_workshops(self.filter.clone(), sort_to_string(&self.sort)) // Get filtered workshops based on the current filter
        };
        egui::Grid::new("headings") // Create a grid for workshop headings
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
        egui::ScrollArea::vertical().show(ui, |ui| { // Display workshops in a scrollable area
            egui::Grid::new("venue_results")
                .num_columns(4)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for workshop in workshops {
                        if ui.button(workshop.name.to_string()).clicked() { // When a workshop name is clicked
                            self.participants.clear();
                            self.support_workers.clear();
                            self.selected_workshop = workshop.clone(); // Set the selected workshop
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

    // The `right_panel_view` method displays details of the selected workshop in the right panel.
    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                if self.selected_workshop.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(self.selected_workshop.name.to_string()); // Display workshop name as heading
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!("Name: {}", self.selected_workshop.name)); // Display workshop details
                        ui.label(format!("Facilitator: {} {}", self.db.get_filtered_support_workers(format!("id = '{}'", self.selected_workshop.facilitator), String::new())[0].first_name, self.db.get_filtered_support_workers(format!("id = '{}'", self.selected_workshop.facilitator), String::new())[0].last_name));
                        ui.label(format!("Venue: {}", self.db.get_filtered_venues(format!("id = '{}'", self.selected_workshop.venue), String::new())[0].name.clone()));
                        ui.label(format!(
                            "start_date: {}",
                            self.selected_workshop.start_date.clone()
                        ));
                        ui.label(format!(
                            "end_date: {}",
                            self.selected_workshop.end_date.clone()
                        ));
                        for participant in self.participants.clone() { // Display participants
                            ui.label(format!("Participant: {} {}", participant.first_name, participant.last_name));
                        }
                        for support_worker in self.support_workers.clone() { // Display support workers
                            ui.label(format!("Support Worker: {} {}", support_worker.first_name, support_worker.last_name));
                        }
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT WORKSHOP"); // Display a prompt when no workshop is selected
                    });
                }
            });
    }

    // The `bottom_menu_view` method displays the bottom menu with options to create, edit, and filter workshops, as well as sort them.
    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(25.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("➕ Create").clicked() { // Button to create a new workshop
                        self.add_window.open = !self.add_window.open;
                    };
                    if ui.button("✏ Edit").clicked() { // Button to edit the selected workshop
                        self.edit_window.open = !self.edit_window.open;
                    };
                    if ui.button("⛭ Filter").clicked() { // Button to open the filter window
                        self.filter_window.open = !self.filter_window.open;
                    };
                    ui.label("Sort: "); // Sorting options
                    egui::ComboBox::from_label("")
                        .selected_text(match self.sort {
                            Sort::AlphabeticalAscending => {String::from("Ascending")} // Ascending order
                            Sort::AlphabeticalDescending => {String::from("Descending")} // Descending order
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

    // The `load_windows_ui` method handles the UI for the add, edit, and filter windows, passing context and UI elements to them.
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ctx); // Load UI for the AddWindow
        self.edit_window.ui(ui, ctx, self.selected_workshop.clone()); // Load UI for the EditWindow with the selected workshop
        self.filter = self.filter_window.ui(ui, ctx); // Load UI for the FilterWindow and update the filter string
    }
}
