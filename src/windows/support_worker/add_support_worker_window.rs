use crate::database_logic::data_structs::SupportWorker;
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

// This struct defines the `AddWindow` UI component which is responsible for
// adding a new support worker into the system.
#[derive(Default)]
pub struct AddWindow {
    // `open` determines if the window is currently open.
    pub open: bool,

    // Database instance used to store the new support worker.
    pub db: DataBase,

    // `reset` determines if the input fields should be reset.
    reset: bool,

    // Fields for entering support worker's details.
    first_name: String,
    last_name: String,
    dob: (bool, NaiveDate),
    phone: String,
    email: String,
    address: (bool, String),
    suburb: (bool, String),
    postcode: (bool, String),
    first_aid: (bool, bool),
    confidentiality_agreement: (bool, bool),
    police_clearance: (bool, bool),
    car_insurance: (bool, bool),
    other_qualifications: (bool, String),
    notes: (bool, String),
}

impl AddWindow {
    // This method handles the UI rendering for the `AddWindow`.
    // It uses `egui` to create the interface for inputting the support worker's details.
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Create Support Worker")
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("add_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            // Field for entering the first name.
                            ui.label("First name:");
                            ui.add(
                                TextEdit::singleline(&mut self.first_name).hint_text("First name"),
                            );
                            ui.end_row();

                            // Field for entering the last name.
                            ui.label("Last name:");
                            ui.add(
                                TextEdit::singleline(&mut self.last_name).hint_text("Last name"),
                            );
                            ui.end_row();

                            // Field for entering the phone number.
                            ui.label("Phone Number:");
                            ui.add(TextEdit::singleline(&mut self.phone).hint_text("Phone Number"));
                            ui.end_row();

                            // Field for entering the email address.
                            ui.label("Email:");
                            ui.add(TextEdit::singleline(&mut self.email).hint_text("Email"));
                            ui.end_row();

                            // Field for selecting the date of birth.
                            ui.label("Date of birth:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.dob.0, |ui| {
                                    ui.add(
                                        DatePickerButton::new(&mut self.dob.1)
                                            .format("%d-%m-%Y")
                                            .highlight_weekends(false)
                                            .id_source("dob"),
                                    );
                                });
                                ui.checkbox(&mut self.dob.0, "Null?");
                            });
                            ui.end_row();

                            // Field for entering the address.
                            ui.label("address:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.address.0, |ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut self.address.1)
                                            .hint_text("address"),
                                    );
                                });
                                ui.checkbox(&mut self.address.0, "Null?");
                            });
                            ui.end_row();

                            // Field for entering the suburb.
                            ui.label("suburb:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.suburb.0, |ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut self.suburb.1)
                                            .hint_text("suburb"),
                                    );
                                });
                                ui.checkbox(&mut self.suburb.0, "Null?");
                            });
                            ui.end_row();

                            // Field for entering the postcode.
                            ui.label("postcode:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.postcode.0, |ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut self.postcode.1)
                                            .hint_text("postcode"),
                                    );
                                });
                                ui.checkbox(&mut self.postcode.0, "Null?");
                            });
                            ui.end_row();

                            // Checkbox for first aid certification.
                            ui.label("first_aid:");
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut self.first_aid.1, "first_aid?");
                                ui.checkbox(&mut self.first_aid.0, "Null?");
                            });
                            ui.end_row();

                            // Checkbox for confidentiality agreement.
                            ui.label("confidentiality_agreement:");
                            ui.horizontal(|ui| {
                                ui.checkbox(
                                    &mut self.confidentiality_agreement.1,
                                    "confidentiality_agreement?",
                                );
                                ui.checkbox(&mut self.confidentiality_agreement.0, "Null?");
                            });
                            ui.end_row();

                            // Checkbox for police clearance.
                            ui.label("police_clearance:");
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut self.police_clearance.1, "police_clearance?");
                                ui.checkbox(&mut self.police_clearance.0, "Null?");
                            });
                            ui.end_row();

                            // Checkbox for car insurance.
                            ui.label("car_insurance:");
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut self.car_insurance.1, "car_insurance?");
                                ui.checkbox(&mut self.car_insurance.0, "Null?");
                            });
                            ui.end_row();

                            // Field for entering other qualifications.
                            ui.label("other_qualifications:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.other_qualifications.0, |ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut self.other_qualifications.1)
                                            .hint_text("other_qualifications"),
                                    );
                                });
                                ui.checkbox(&mut self.other_qualifications.0, "Null?");
                            });
                            ui.end_row();

                            // Field for entering notes.
                            ui.label("notes:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.notes.0, |ui| {
                                    ui.add(
                                        TextEdit::singleline(&mut self.notes.1).hint_text("notes"),
                                    );
                                });
                                ui.checkbox(&mut self.notes.0, "Null?");
                            });
                            ui.end_row();
                        });
                });

                // Add and Reset buttons at the bottom of the window.
                ui.separator();
                ui.horizontal(|ui| {
                    // Button to add the support worker to the database.
                    if ui.button("➕ ADD").clicked() {
                        // todo: need to add data validation.
                        let new_support_worker = SupportWorker {
                            id: None,
                            first_name: self.first_name.clone(),
                            last_name: self.last_name.clone(),
                            phone: self.phone.clone(),
                            email: self.email.clone(),
                            dob: (!self.dob.0).then_some(self.dob.1),
                            address: (!self.address.0).then(|| self.address.1.clone()),
                            suburb: (!self.suburb.0).then(|| self.suburb.1.clone()),
                            postcode: (!self.postcode.0).then(|| self.postcode.1.clone()),
                            first_aid: (!self.first_aid.0).then_some(self.first_aid.1),
                            confidentiality_agreement: (!self.confidentiality_agreement.0)
                                .then_some(self.confidentiality_agreement.1),
                            police_clearance: (!self.police_clearance.0)
                                .then_some(self.police_clearance.1),
                            car_insurance: (!self.car_insurance.0).then_some(self.car_insurance.1),
                            other_qualifications: (!self.other_qualifications.0)
                                .then(|| self.other_qualifications.1.clone()),
                            notes: (!self.notes.0).then(|| self.notes.1.clone()),
                        };
                        // Add the new support worker to the database.
                        self.db.add_support_worker(new_support_worker).unwrap();
                    };

                    // Button to reset the input fields.
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;
                    };
                });
            });

        // Reset the values if the window is closed or reset is triggered.
        if !self.open | self.reset {
            self.reset_values();
        }
    }

    // Resets all input fields to their default values.
    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
