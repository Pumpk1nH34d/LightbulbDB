// Import necessary modules from the `egui` crate.
use egui::{Context, TextEdit, Ui};

// Define the `FilterWindow` struct with default values.
#[derive(Default)]
pub struct FilterWindow {
    // Fields related to the filter window's state and user input.
    pub open: bool,                // Indicates if the filter window is open.
    reset: bool,                   // Tracks if the reset button was clicked.
    filter: String,                // Stores the filter query.

    // User input fields.
    first_name: String,
    last_name: String,
    phone: String,
    email: String,
    address: String,
    suburb: String,
    postcode: String,
    other_qualifications: String,
    notes: String,
}

// Implement methods for the `FilterWindow` struct.
impl FilterWindow {
    // Define the `ui` method to handle the UI and filter logic.
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) -> String {
        // Create and display a new window for the filter interface.
        egui::Window::new("Filter Venue")
            .open(&mut self.open)  // Bind the window's open state to the `open` field.
            .show(ctx, |ui| {      // Show the UI elements within the window.
                egui::ScrollArea::vertical().show(ui, |ui| { // Create a vertical scroll area.
                    egui::Grid::new("filter_venue_grid")
                        .num_columns(2)                    // Set the number of columns in the grid.
                        .spacing([40.0, 4.0])              // Define the spacing between grid elements.
                        .striped(true)                     // Enable striped rows for better visibility.
                        .show(ui, |ui| {                   // Show the grid with labels and text inputs.
                            ui.label("First name:");
                            ui.add(
                                TextEdit::singleline(&mut self.first_name)
                                    .hint_text("First name"),
                            );
                            ui.end_row(); // End the current row in the grid.

                            ui.label("Last name:");
                            ui.add(
                                TextEdit::singleline(&mut self.last_name)
                                    .hint_text("Last name"),
                            );
                            ui.end_row();

                            ui.label("Phone Number:");
                            ui.add(
                                TextEdit::singleline(&mut self.phone).hint_text("Phone Number"),
                            );
                            ui.end_row();

                            ui.label("Email:");
                            ui.add(TextEdit::singleline(&mut self.email).hint_text("Email"));
                            ui.end_row();

                            ui.label("Address:");
                            ui.add(
                                TextEdit::singleline(&mut self.address)
                                    .hint_text("Address"),
                            );
                            ui.end_row();

                            ui.label("Suburb:");
                            ui.add(
                                TextEdit::singleline(&mut self.suburb)
                                    .hint_text("Suburb"),
                            );
                            ui.end_row();

                            ui.label("Postcode:");
                            ui.add(
                                TextEdit::singleline(&mut self.postcode)
                                    .hint_text("Postcode"),
                            );
                            ui.end_row();

                            ui.label("Other Qualifications:");
                            ui.add(
                                TextEdit::singleline(&mut self.other_qualifications)
                                    .hint_text("Other Qualifications"),
                            );
                            ui.end_row();

                            ui.label("Notes:");
                            ui.add(
                                TextEdit::singleline(&mut self.notes)
                                    .hint_text("Notes"),
                            );
                            ui.end_row();
                        });
                });
                ui.separator(); // Add a separator line for visual clarity.
                ui.horizontal(|ui| { // Create a horizontal layout for the buttons.
                    if ui.button("✔ APPLY").clicked() {
                        // Apply button logic: build the filter query string based on user input.
                        let mut filter = String::new();
                        if !self.first_name.is_empty() {
                            filter += &format!("first_name = '{}' AND ", self.first_name)
                        };
                        if !self.last_name.is_empty() {
                            filter += &format!("last_name = '{}' AND ", self.last_name)
                        }
                        if !self.phone.is_empty() {
                            filter += &format!("phone = '{}' AND ", self.phone)
                        }
                        if !self.email.is_empty() {
                            filter += &format!("email = '{}' AND ", self.email)
                        }
                        if !self.address.is_empty() {
                            filter += &format!("address = '{}' AND ", self.address)
                        }
                        if !self.suburb.is_empty() {
                            filter += &format!("suburb = '{}' AND ", self.suburb)
                        }
                        if !self.postcode.is_empty() {
                            filter += &format!("postcode = '{}' AND ", self.postcode)
                        }
                        if !self.other_qualifications.is_empty() {
                            filter += &format!("other_qualifications = '{}' AND ", self.other_qualifications)
                        }
                        if !self.notes.is_empty() {
                            filter += &format!("notes = '{}' AND ", self.notes)
                        }
                        if !filter.is_empty() {
                            filter.truncate(filter.len() - 5) // Remove the trailing " AND " from the filter string.
                        }
                        self.filter = filter;
                    }
                    if ui.button("🔃 Reset").clicked() {
                        // Reset button logic: set the `reset` flag to true.
                        self.reset = true;
                    };
                });
            });
        if self.reset {
            self.reset_values(); // Call the `reset_values` method if the reset flag is set.
        };
        self.filter.clone() // Return the filter string.
    }

    // Define the `reset_values` method to reset all input fields to their default values.
    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open); // Reset the struct to its default state while preserving the `open` state.
    }
}
