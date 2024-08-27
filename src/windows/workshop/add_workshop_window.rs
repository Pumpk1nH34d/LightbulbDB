use crate::database_logic::data_structs::{SupportWorker, Venue, Workshop}; // Import necessary data structures from the database logic module
use crate::database_logic::database::DataBase; // Import the database struct
use chrono::NaiveDate; // Import NaiveDate from the chrono crate for date handling
use egui::{Context, TextEdit}; // Import Context and TextEdit from the egui crate for UI rendering
use egui_extras::DatePickerButton; // Import DatePickerButton from egui_extras for date selection UI

// Enum to manage the different views related to foreign key fields in the UI
#[derive(Default)]
pub enum ForeignView {
    #[default]
    Facilitator, // View for selecting a facilitator
    Venue, // View for selecting a venue
    Participants, // View for selecting participants
    SupportWorkers // View for selecting support workers
}

// Struct representing the state and logic of the "Add Workshop" window in the UI
#[derive(Default)]
pub struct AddWindow {
    pub open: bool, // Indicates whether the window is open
    pub db: DataBase, // The database connection
    reset: bool, // Flag to reset the window's values
    right_view_selected: Option<ForeignView>, // Currently selected foreign view

    name: String, // Workshop name
    support_workers: Vec<i32>, // Selected support workers by ID
    support_worker_filter: String, // Filter for searching support workers
    participants: Vec<i32>, // Selected participants by ID
    participant_filter: String, // Filter for searching participants
    facilitator: SupportWorker, // Selected facilitator
    facilitator_filter: String, // Filter for searching facilitators
    venue: Venue, // Selected venue
    venue_filter: String, // Filter for searching venues
    start_date: NaiveDate, // Workshop start date
    end_date: NaiveDate, // Workshop end date
}

impl AddWindow {
    // Method to render the UI for adding a workshop
    pub fn ui(&mut self, ctx: &Context) {
        // Create a new window titled "Create Workshop"
        egui::Window::new("Create Workshop")
            .open(&mut self.open) // Control window visibility
            .max_height(110.0) // Set maximum height
            .default_width(500.0) // Set default width
            .resizable(true) // Allow window resizing
            .show(ctx, |ui| {
                // Create a right side panel for displaying foreign key options
                egui::SidePanel::right("create_workshop_right_panel")
                    .resizable(true)
                    .default_width(150.0)
                    .show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // Create a grid to display filtered results
                            egui::Grid::new("foreign_add_filter_results")
                                .num_columns(6)
                                .spacing([30.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    if self.right_view_selected.is_some() {
                                        // Match the selected view to display relevant options
                                        match self.right_view_selected.as_ref().unwrap() {
                                            ForeignView::Facilitator => {
                                                let facilitators =
                                                    if self.facilitator_filter.is_empty() {
                                                        // Get all support workers if no filter is applied
                                                        self.db.get_all_support_workers(
                                                            String::from(""),
                                                        )
                                                    } else {
                                                        // Get filtered support workers
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
                                                // Display facilitator options
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
                                                    ui.end_row();
                                                }
                                            }
                                            ForeignView::Venue => {
                                                let venues = if self.venue_filter.is_empty() {
                                                    // Get all venues if no filter is applied
                                                    self.db.get_all_venues(String::from(""))
                                                } else {
                                                    // Get filtered venues
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
                                                // Display venue options
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
                                                        // Get all participants if no filter is applied
                                                        self.db
                                                            .get_all_participants(String::from(""))
                                                    } else {
                                                        // Get filtered participants
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
                                                // Display participant options
                                                for participant in participants {
                                                    let id = participant.id.unwrap();
                                                    if self.participants.contains(&id) {
                                                        // Display selected participants with a checkbox
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
                                                        // Display unselected participants
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
                                                        // Get all support workers if no filter is applied
                                                        self.db
                                                            .get_all_support_workers(String::from(""))
                                                    } else {
                                                        // Get filtered support workers
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
                                                // Display support worker options
                                                for support_worker in support_workers {
                                                    let id = support_worker.id.unwrap();
                                                    if self.support_workers.contains(&id) {
                                                        // Display selected support workers with a checkbox
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
                                                        // Display unselected support workers
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
                                        ui.label("SELECT AN INPUT"); // Prompt if no view is selected
                                    }
                                });
                        });
                    });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Create a grid for workshop creation form
                    egui::Grid::new("create_workshop_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Name:"); // Label for workshop name
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name")); // Input for workshop name
                            ui.end_row();
                            ui.label("Facilitator:"); // Label for facilitator
                            ui.horizontal(|ui| {
                                if self.facilitator.id.is_some() {
                                    ui.label(format!(
                                        "{} {}",
                                        self.facilitator.first_name, self.facilitator.last_name
                                    )); // Display selected facilitator
                                }
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.facilitator_filter)
                                            .hint_text("facilitator"),
                                    )
                                    .has_focus()
                                {
                                    self.right_view_selected = Some(ForeignView::Facilitator); // Set view to facilitator
                                }
                            });
                            ui.end_row();
                            ui.label("Participants:"); // Label for participants
                            if ui
                                .add(
                                    TextEdit::singleline(&mut self.participant_filter)
                                        .hint_text("participant"),
                                )
                                .has_focus()
                            {
                                self.right_view_selected = Some(ForeignView::Participants); // Set view to participants
                            }
                            ui.end_row();
                            ui.label("Support Workers:"); // Label for support workers
                            if ui
                                .add(
                                    TextEdit::singleline(&mut self.support_worker_filter)
                                        .hint_text("participant"),
                                )
                                .has_focus()
                            {
                                self.right_view_selected = Some(ForeignView::SupportWorkers); // Set view to support workers
                            }
                            ui.end_row();
                            ui.label("Venue:"); // Label for venue
                            ui.horizontal(|ui| {
                                if self.venue.id.is_some() {
                                    ui.label(self.venue.name.to_string()); // Display selected venue
                                }
                                if ui
                                    .add(
                                        TextEdit::singleline(&mut self.venue_filter)
                                            .hint_text("venue"),
                                    )
                                    .has_focus()
                                {
                                    self.right_view_selected = Some(ForeignView::Venue); // Set view to venue
                                }
                            });
                            ui.end_row();
                            ui.label("Start Date:"); // Label for start date
                            ui.add(
                                DatePickerButton::new(&mut self.start_date)
                                    .format("%d-%m-%Y")
                                    .highlight_weekends(false)
                                    .id_source("start_date"),
                            ); // Date picker for start date
                            ui.end_row();
                            ui.label("End Date:"); // Label for end date
                            ui.add(
                                DatePickerButton::new(&mut self.end_date)
                                    .format("%d-%m-%Y")
                                    .highlight_weekends(false)
                                    .id_source("end_date"),
                            ); // Date picker for end date
                            ui.end_row();
                        });
                });
                ui.separator(); // Add a separator between the form and buttons
                ui.horizontal(|ui| {
                    // Add and Reset buttons
                    // todo: need to add data validation.
                    if ui.button("➕ ADD").clicked() {
                        // On Add button click, create a new workshop entry
                        let new_workshop = Workshop {
                            id: None,
                            name: self.name.clone(),
                            facilitator: self.facilitator.id.unwrap(),
                            venue: self.venue.id.unwrap(),
                            start_date: self.start_date,
                            end_date: self.end_date,
                        };
                        self.db.add_workshop(new_workshop).unwrap(); // Add the workshop to the database
                        let workshop_id = self.db.get_filtered_workshops(format!("name = '{}' AND facilitator = '{}' AND venue = '{}' AND start_date = '{}' AND end_date = '{}'", &self.name, &self.facilitator.id.unwrap(), &self.venue.id.unwrap(), &self.start_date.to_string(), &self.end_date.to_string()), String::new())[0].id;
                        self.db.add_participants_to_workshop(&self.participants, workshop_id.unwrap()).unwrap(); // Add selected participants to the workshop
                        self.db.add_support_workers_to_workshop(&self.support_workers, workshop_id.unwrap()).unwrap(); // Add selected support workers to the workshop
                    };
                    if ui.button("🔃 Reset").clicked() {
                        // On Reset button click, reset the form
                        self.reset = true;
                    };
                });
            });
        if !self.open | self.reset {
            // Reset values if the window is closed or reset flag is true
            self.reset_values();
        }
    }

    // Method to reset the form values to their defaults
    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
