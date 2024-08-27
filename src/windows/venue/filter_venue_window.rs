use egui::{Context, TextEdit, Ui};

// The `FilterWindow` struct represents the state of the filter window UI.
// It includes fields for the visibility of the window (`open`), whether a reset has been triggered (`reset`),
// and various fields for the filtering criteria such as `name`, `address`, `suburb`, etc.
#[derive(Default)]
pub struct FilterWindow {
    pub open: bool, // Controls whether the window is open.
    reset: bool,    // Tracks whether the reset button was clicked.

    // Fields for the filtering criteria.
    filter: String,

    name: String,
    address: String,
    suburb: String,
    postcode: String,
    state: String,
    description: String,
    contact_person_name: String,
    contact_person_phone: String,
    venue_phone_number: String,
    price: String,
    notes: String,
}

impl FilterWindow {
    // This method defines the user interface (UI) for the filter window.
    // It returns the filter string that contains the filtering criteria selected by the user.
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) -> String {
        // Create a new window with the title "Filter Venue".
        egui::Window::new("Filer Venue")
            .open(&mut self.open) // Set the window open status based on the `open` field.
            .show(ctx, |ui| {
                // Display the filtering fields in a scrollable area.
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Create a grid layout with two columns and spacing between elements.
                    egui::Grid::new("filter_venue_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            // Display labels and text input fields for each filtering criterion.
                            ui.label("Name:");
                            ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
                            ui.end_row();
                            ui.label("address:");
                            ui.add(TextEdit::singleline(&mut self.address).hint_text("address"));
                            ui.end_row();
                            ui.label("suburb:");
                            ui.add(TextEdit::singleline(&mut self.suburb).hint_text("suburb"));
                            ui.end_row();
                            ui.label("postcode:");
                            ui.add(TextEdit::singleline(&mut self.postcode).hint_text("postcode"));
                            ui.end_row();
                            ui.label("State:");
                            ui.add(TextEdit::singleline(&mut self.state).hint_text("postcode"));
                            ui.end_row();
                            ui.label("description:");
                            ui.add(
                                TextEdit::singleline(&mut self.description)
                                    .hint_text("description"),
                            );
                            ui.end_row();
                            ui.label("contact_person_name:");
                            ui.add(
                                TextEdit::singleline(&mut self.contact_person_name)
                                    .hint_text("contact_person_name"),
                            );
                            ui.end_row();
                            ui.label("contact_person_phone:");
                            ui.add(
                                TextEdit::singleline(&mut self.contact_person_phone)
                                    .hint_text("contact_person_phone"),
                            );
                            ui.end_row();
                            ui.label("venue_phone_number:");
                            ui.add(
                                TextEdit::singleline(&mut self.venue_phone_number)
                                    .hint_text("venue_phone_number"),
                            );
                            ui.end_row();
                            ui.label("price:");
                            ui.add(TextEdit::singleline(&mut self.price).hint_text("price"));
                            ui.end_row();
                            ui.label("notes:");
                            ui.add(TextEdit::singleline(&mut self.notes).hint_text("notes"));
                            ui.end_row();
                        });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    // Handle the "APPLY" button click event.
                    if ui.button("✔ APPLY").clicked() {
                        // Build the filter string based on the filled fields.
                        let mut filter = String::new();
                        if !self.name.is_empty() {
                            filter += &format!("name = '{}' AND ", self.name)
                        };
                        if !self.address.is_empty() {
                            filter += &format!("address = '{}' AND ", self.address)
                        }
                        if !self.suburb.is_empty() {
                            filter += &format!("suburb = '{}' AND ", self.suburb)
                        }
                        if !self.postcode.is_empty() {
                            filter += &format!("postcode = '{}' AND ", self.postcode)
                        }
                        if !self.state.is_empty() {
                            filter += &format!("state = '{}' AND ", self.state)
                        }
                        if !self.description.is_empty() {
                            filter += &format!("description = '{}' AND ", self.description)
                        }
                        if !self.contact_person_name.is_empty() {
                            filter += &format!("contact_person_name = '{}' AND ", self.contact_person_name)
                        }
                        if !self.contact_person_phone.is_empty() {
                            filter += &format!("contact_person_phone = '{}' AND ", self.contact_person_phone)
                        }
                        if !self.venue_phone_number.is_empty() {
                            filter += &format!("venue_phone_number = '{}' AND ", self.venue_phone_number)
                        }
                        if !self.price.is_empty() {
                            filter += &format!("price = '{}' AND ", self.price)
                        }
                        if !self.notes.is_empty() {
                            filter += &format!("notes = '{}' AND ", self.notes)
                        }
                        // Remove the last " AND " from the filter string if it exists.
                        if !filter.is_empty() {
                            filter.truncate(filter.len() - 5)
                        }
                        // Store the generated filter string in the `filter` field.
                        self.filter = filter;
                    }
                    // Handle the "Reset" button click event.
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;
                    };
                });
            });
        // If the reset flag is set, reset the fields to their default values.
        if self.reset {
            self.reset_values();
        };
        // Return the generated filter string.
        self.filter.clone()
    }

    // This method resets all fields in the filter window to their default values.
    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
