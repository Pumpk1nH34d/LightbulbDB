// Importing the necessary modules from the `egui` crate.
use egui::{Context, TextEdit, Ui};

// This struct represents a filter window used in the UI. It stores various fields
// related to user input and the filter's open and reset state.
#[derive(Default)]
pub struct FilterWindow {
    pub open: bool,  // Indicates whether the filter window is open.
    reset: bool,     // Indicates whether the filter values should be reset.

    // Fields for storing user input data.
    filter: String,
    first_name: String,
    last_name: String,
    medicare: String,
    phone: String,
    email: String,
    address: String,
    suburb: String,
    postcode: String,
    medical_notes: String,
    dietary_notes: String,
    physical_notes: String,
    other_notes: String,
    support_ratio: String,
    private_health_insurer: String,
    private_health_number: String,
    communication_preference: String,
    ndis_plan_number: String,
}

// Implementation block for the `FilterWindow` struct.
impl FilterWindow {
    // Function to render the UI for the filter window.
    // Takes a mutable reference to `self`, a mutable reference to the UI, and the context.
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) -> String {
        // Creating a new window with the title "Filer Venue".
        egui::Window::new("Filer Venue")
            .open(&mut self.open)  // Set whether the window is open.
            .show(ctx, |ui| {      // Display the window with the given context.
                // Creating a vertical scroll area inside the window.
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Creating a grid layout for the input fields.
                    egui::Grid::new("filter_venue_grid")
                        .num_columns(2)    // Setting the number of columns to 2.
                        .spacing([40.0, 4.0])  // Setting the spacing between elements.
                        .striped(true)     // Enabling striped rows.
                        .show(ui, |ui| {
                            // Adding labels and text fields for each input field.
                            ui.label("First name:");
                            ui.add(
                                TextEdit::singleline(&mut self.first_name)
                                    .hint_text("First name"),
                            );
                            ui.end_row();
                            ui.label("Last name:");
                            ui.add(
                                TextEdit::singleline(&mut self.last_name)
                                    .hint_text("Last name"),
                            );
                            ui.end_row();
                            ui.label("Phone number:");
                            ui.add(
                                TextEdit::singleline(&mut self.phone)
                                    .hint_text("Phone number"),
                            );
                            ui.end_row();
                            ui.label("Email:");
                            ui.add(
                                TextEdit::singleline(&mut self.email)
                                    .hint_text("Email"),
                            );
                            ui.end_row();
                            ui.label("Medicare Number:");
                            ui.add(
                                TextEdit::singleline(&mut self.medicare).hint_text("Medicare"),
                            );
                            ui.end_row();
                            ui.label("medical_notes:");
                            ui.add(
                                TextEdit::singleline(&mut self.medical_notes)
                                    .hint_text("medical_notes"),
                            );
                            ui.end_row();
                            ui.label("dietary_notes:");
                            ui.add(
                                TextEdit::singleline(&mut self.dietary_notes)
                                    .hint_text("dietary_notes"),
                            );
                            ui.end_row();
                            ui.label("physical_notes:");
                            ui.add(
                                TextEdit::singleline(&mut self.physical_notes)
                                    .hint_text("physical_notes"),
                            );
                            ui.end_row();
                            ui.label("other_notes:");
                            ui.add(
                                TextEdit::singleline(&mut self.other_notes)
                                    .hint_text("other_notes"),
                            );
                            ui.end_row();
                            ui.label("support_ratio:");
                            ui.add(
                                TextEdit::singleline(&mut self.support_ratio)
                                    .hint_text("support_ratio"),
                            );
                            ui.end_row();
                            ui.label("photo_permission:");
                            ui.end_row();
                            ui.label("private_health_number:");
                            ui.add(
                                TextEdit::singleline(&mut self.private_health_number)
                                    .hint_text("private_health_number"),
                            );
                            ui.end_row();
                            ui.label("communication_preference:");
                            ui.add(
                                TextEdit::singleline(
                                    &mut self.communication_preference,
                                )
                                    .hint_text("communication_preference"),
                            );
                            ui.end_row();
                            ui.label("ndis_plan_number:");
                            ui.add(
                                TextEdit::singleline(&mut self.ndis_plan_number)
                                    .hint_text("ndis_plan_number"),
                            );
                            ui.end_row();
                        });
                });
                ui.separator();  // Adding a separator in the UI.
                // Adding buttons for applying the filter and resetting the values.
                ui.horizontal(|ui| {
                    if ui.button("✔ APPLY").clicked() {
                        // Building the filter string based on non-empty input fields.
                        let mut filter = String::new();
                        if !self.first_name.is_empty() {
                            filter += &format!("first_name = '{}' AND ", self.first_name)
                        };
                        if !self.last_name.is_empty() {
                            filter += &format!("last_name = '{}' AND ", self.last_name)
                        }
                        if !self.medicare.is_empty() {
                            filter += &format!("medicare = '{}' AND ", self.medicare)
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
                        if !self.phone.is_empty() {
                            filter += &format!("phone = '{}' AND ", self.phone)
                        }
                        if !self.email.is_empty() {
                            filter += &format!("email = '{}' AND ", self.email)
                        }
                        if !self.medical_notes.is_empty() {
                            filter += &format!("medical_notes = '{}' AND ", self.medical_notes)
                        }
                        if !self.dietary_notes.is_empty() {
                            filter += &format!("dietary_notes = '{}' AND ", self.dietary_notes)
                        }
                        if !self.physical_notes.is_empty() {
                            filter += &format!("physical_notes = '{}' AND ", self.physical_notes)
                        }
                        if !self.other_notes.is_empty() {
                            filter += &format!("other_notes = '{}' AND ", self.other_notes)
                        }
                        if !self.support_ratio.is_empty() {
                            filter += &format!("support_ratio = '{}' AND ", self.support_ratio)
                        }
                        if !self.private_health_insurer.is_empty() {
                            filter += &format!("private_health_insurer = '{}' AND ", self.private_health_insurer)
                        }
                        if !self.private_health_number.is_empty() {
                            filter += &format!("private_health_number = '{}' AND ", self.private_health_number)
                        }
                        if !self.communication_preference.is_empty() {
                            filter += &format!("communication_preference = '{}' AND ", self.communication_preference)
                        }
                        if !self.ndis_plan_number.is_empty() {
                            filter += &format!("ndis_plan_number = '{}' AND ", self.ndis_plan_number)
                        }
                        // Removing the trailing " AND " from the filter string.
                        if !filter.is_empty() {
                            filter.truncate(filter.len() - 5)
                        }
                        self.filter = filter;  // Updating the filter string.
                    }
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;  // Setting the reset flag to true when the reset button is clicked.
                    };
                });
            });
        if self.reset {
            self.reset_values();  // Resetting the values if the reset flag is set.
        };
        self.filter.clone()  // Returning the filter string.
    }

    // Function to reset the filter window's values to their default states.
    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
