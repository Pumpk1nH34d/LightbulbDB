use crate::database_logic::data_structs::SupportWorker;
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

#[derive(Default)]
pub struct EditWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    changed: bool,
    support_worker_check: SupportWorker,

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

impl EditWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context, support_worker: SupportWorker) {
        if self.support_worker_check.id != support_worker.id {
            self.support_worker_check = support_worker.clone();
            self.first_name = support_worker.first_name;
            self.last_name = support_worker.last_name;
            self.email = support_worker.email;
            self.phone = support_worker.phone;
            match support_worker.dob {
                None => self.dob.0 = true,
                Some(value) => self.dob = (false, value),
            }
            match support_worker.address {
                None => self.address.0 = true,
                Some(value) => self.address = (false, value),
            }
            match support_worker.suburb {
                None => self.suburb.0 = true,
                Some(value) => self.suburb = (false, value),
            }
            match support_worker.postcode {
                None => self.postcode.0 = true,
                Some(value) => self.postcode = (false, value),
            }
            match support_worker.first_aid {
                None => self.first_aid.0 = true,
                Some(value) => self.first_aid = (false, value),
            }
            match support_worker.confidentiality_agreement {
                None => self.confidentiality_agreement.0 = true,
                Some(value) => self.confidentiality_agreement = (false, value),
            }
            match support_worker.police_clearance {
                None => self.police_clearance.0 = true,
                Some(value) => self.police_clearance = (false, value),
            }
            match support_worker.car_insurance {
                None => self.car_insurance.0 = true,
                Some(value) => self.car_insurance = (false, value),
            }
            match support_worker.other_qualifications {
                None => self.other_qualifications.0 = true,
                Some(value) => self.other_qualifications = (false, value),
            }
            match support_worker.notes {
                None => self.notes.0 = true,
                Some(value) => self.notes = (false, value),
            }
        }
        if !self.open {
            self.changed = false;
        };
        egui::Window::new("Edit Support Worker")
            .open(&mut self.open)
            .show(ctx, |ui| {
                if self.changed {
                    ui.label("DONE ✔");
                    ui.label("Reselect Participant to see changes.");
                } else if self.support_worker_check.id.is_none() {
                    ui.label("Select a Participant to EDIT");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("edit_support_worker")
                            .num_columns(2)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
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
                                ui.label("Phone Number:");
                                ui.add(
                                    TextEdit::singleline(&mut self.phone).hint_text("Phone Number"),
                                );
                                ui.end_row();
                                ui.label("Email:");
                                ui.add(TextEdit::singleline(&mut self.phone).hint_text("Email"));
                                ui.end_row();
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
                                ui.label("first_aid:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.first_aid.1, "first_aid?");
                                    ui.checkbox(&mut self.first_aid.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("confidentiality_agreement:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(
                                        &mut self.confidentiality_agreement.1,
                                        "confidentiality_agreement?",
                                    );
                                    ui.checkbox(&mut self.confidentiality_agreement.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("police_clearance:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.police_clearance.1, "police_clearance?");
                                    ui.checkbox(&mut self.police_clearance.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("car_insurance:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.car_insurance.1, "Photo car_insurance?");
                                    ui.checkbox(&mut self.car_insurance.0, "Null?");
                                });
                                ui.end_row();
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

                                ui.label("notes:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.notes.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.notes.1)
                                                .hint_text("other_qualifications"),
                                        );
                                    });
                                    ui.checkbox(&mut self.notes.0, "Null?");
                                });
                                ui.end_row();
                            });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✔ Confirm").clicked() {
                            let edited_support_worker = SupportWorker {
                                id: self.support_worker_check.id,
                                first_name: self.first_name.clone(),
                                last_name: self.last_name.clone(),
                                phone: self.phone.clone(),
                                email: self.email.clone(),
                                dob: (!self.dob.0).then_some(self.dob.1),
                                address: (!self.address.0).then(|| self.address.1.clone()),
                                suburb: (!self.suburb.0).then(|| self.suburb.1.clone()),
                                postcode: (!self.postcode.0).then(|| self.postcode.1.clone()),
                                first_aid: (!self.first_aid.0).then(|| self.first_aid.1.clone()),
                                confidentiality_agreement: (!self.confidentiality_agreement.0)
                                    .then(|| self.confidentiality_agreement.1.clone()),
                                police_clearance: (!self.police_clearance.0)
                                    .then(|| self.police_clearance.1.clone()),
                                car_insurance: (!self.car_insurance.0)
                                    .then(|| self.car_insurance.1.clone()),
                                other_qualifications: (!self.other_qualifications.0)
                                    .then(|| self.other_qualifications.1.clone()),
                                notes: (!self.notes.0).then(|| self.notes.1.clone()),
                            };
                            self.db.edit_support_worker(edited_support_worker).unwrap();
                            self.support_worker_check.id = None;
                            self.changed = true;
                        };
                        if ui.button("❌ Delete").clicked() {
                            self.db
                                .delete_support_worker(self.support_worker_check.id.unwrap());
                            self.support_worker_check.id = None;
                            self.changed = true;
                        };
                    });
                }
            });
    }
}
