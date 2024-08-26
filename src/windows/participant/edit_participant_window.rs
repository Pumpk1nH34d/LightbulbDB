use crate::database_logic::data_structs::Participant;
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;

#[derive(Default)]
pub struct EditWindow {
    pub open: bool,
    pub db: DataBase,
    changed: bool,
    participant_check: Participant,
    
    first_name: String,
    last_name: String,
    medicare: String,
    dob: (bool, NaiveDate),
    phone: (bool, String),
    email: (bool, String),
    address: (bool, String),
    suburb: (bool, String),
    postcode: (bool, String),
    medical_notes: (bool, String),
    dietary_notes: (bool, String),
    physical_notes: (bool, String),
    other_notes: (bool, String),
    support_ratio: (bool, String),
    photo_permission: (bool, bool),
    private_hospital_preference: (bool, bool),
    private_health_insurer: (bool, String),
    private_health_number: (bool, String),
    communication_preference: (bool, String),
    ndis_plan_number: (bool, String),
    ndis_plan_start_date: (bool, NaiveDate),
    ndis_plan_end_date: (bool, NaiveDate),
    core_funding: (bool, bool),
    capacity_building_funding: (bool, bool),
    self_managed: (bool, bool),
    plan_managed: (bool, bool),
}

impl EditWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context, participant: Participant) {
        if self.participant_check.id != participant.id {
            self.participant_check = participant.clone();
            self.first_name = participant.first_name;
            self.last_name = participant.last_name;
            self.medicare = participant.medicare_number;
            match participant.dob {
                None => self.dob.0 = true,
                Some(value) => self.dob = (false, value),
            }
            match participant.phone {
                None => self.phone.0 = true,
                Some(value) => self.phone = (false, value),
            }
            match participant.email {
                None => self.email.0 = true,
                Some(value) => self.email = (false, value),
            }
            match participant.address {
                None => self.address.0 = true,
                Some(value) => self.address = (false, value),
            }
            match participant.suburb {
                None => self.suburb.0 = true,
                Some(value) => self.suburb = (false, value),
            }
            match participant.postcode {
                None => self.postcode.0 = true,
                Some(value) => self.postcode = (false, value),
            }
            match participant.medical_notes {
                None => self.medical_notes.0 = true,
                Some(value) => self.medical_notes = (false, value),
            }
            match participant.dietary_notes {
                None => self.dietary_notes.0 = true,
                Some(value) => self.dietary_notes = (false, value),
            }
            match participant.physical_notes {
                None => self.physical_notes.0 = true,
                Some(value) => self.physical_notes = (false, value),
            }
            match participant.other_notes {
                None => self.other_notes.0 = true,
                Some(value) => self.other_notes = (false, value),
            }
            match participant.support_ratio {
                None => self.support_ratio.0 = true,
                Some(value) => self.support_ratio = (false, value),
            }
            match participant.photo_permission {
                None => self.photo_permission.0 = true,
                Some(value) => self.photo_permission = (false, value),
            }
            match participant.private_hospital_preference {
                None => self.private_hospital_preference.0 = true,
                Some(value) => self.private_hospital_preference = (false, value),
            }
            match participant.private_health_insurer {
                None => self.private_health_insurer.0 = true,
                Some(value) => self.private_health_insurer = (false, value),
            }
            match participant.private_health_number {
                None => self.private_health_number.0 = true,
                Some(value) => self.private_health_number = (false, value),
            }
            match participant.communication_preference {
                None => self.communication_preference.0 = true,
                Some(value) => self.communication_preference = (false, value),
            }
            match participant.ndis_plan_number {
                None => self.ndis_plan_number.0 = true,
                Some(value) => self.ndis_plan_number = (false, value),
            }
            match participant.ndis_plan_start_date {
                None => self.ndis_plan_start_date.0 = true,
                Some(value) => self.ndis_plan_start_date = (false, value),
            }
            match participant.ndis_plan_end_date {
                None => self.ndis_plan_end_date.0 = true,
                Some(value) => self.ndis_plan_end_date = (false, value),
            }
            match participant.core_funding {
                None => self.core_funding.0 = true,
                Some(value) => self.core_funding = (false, value),
            }
            match participant.capacity_building_funding {
                None => self.capacity_building_funding.0 = true,
                Some(value) => self.capacity_building_funding = (false, value),
            }
            match participant.self_managed {
                None => self.self_managed.0 = true,
                Some(value) => self.self_managed = (false, value),
            }
            match participant.plan_managed {
                None => self.plan_managed.0 = true,
                Some(value) => self.plan_managed = (false, value),
            }
        }
        if !self.open {
            self.changed = false;
        };
        egui::Window::new("Edit Participant")
            .open(&mut self.open)
            .show(ctx, |ui| {
                if self.changed {
                    ui.label("DONE ✔");
                    ui.label("Reselect Participant to see changes.");
                } else if self.participant_check.id.is_none() {
                    ui.label("Select a Participant to EDIT");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("edit_participant")
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
                                ui.label("Phone number:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.phone.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.phone.1)
                                                .hint_text("Phone number"),
                                        );
                                    });
                                    ui.checkbox(&mut self.phone.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("Email:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.email.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.email.1)
                                                .hint_text("Email"),
                                        );
                                    });
                                    ui.checkbox(&mut self.email.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("Medicare Number:");
                                ui.add(
                                    TextEdit::singleline(&mut self.medicare).hint_text("Medicare"),
                                );
                                ui.end_row();
                                ui.label("medical_notes:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.medical_notes.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.medical_notes.1)
                                                .hint_text("medical_notes"),
                                        );
                                    });
                                    ui.checkbox(&mut self.medical_notes.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("dietary_notes:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.dietary_notes.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.dietary_notes.1)
                                                .hint_text("dietary_notes"),
                                        );
                                    });
                                    ui.checkbox(&mut self.dietary_notes.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("physical_notes:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.physical_notes.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.physical_notes.1)
                                                .hint_text("physical_notes"),
                                        );
                                    });
                                    ui.checkbox(&mut self.physical_notes.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("other_notes:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.other_notes.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.other_notes.1)
                                                .hint_text("other_notes"),
                                        );
                                    });
                                    ui.checkbox(&mut self.other_notes.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("support_ratio:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.support_ratio.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.support_ratio.1)
                                                .hint_text("support_ratio"),
                                        );
                                    });
                                    ui.checkbox(&mut self.support_ratio.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("photo_permission:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.photo_permission.1, "Photo Permission?");
                                    ui.checkbox(&mut self.photo_permission.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("private_hospital_preference:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(
                                        &mut self.private_hospital_preference.1,
                                        "private_hospital_preference?",
                                    );
                                    ui.checkbox(&mut self.private_hospital_preference.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("private_health_number:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.private_health_number.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.private_health_number.1)
                                                .hint_text("private_health_number"),
                                        );
                                    });
                                    ui.checkbox(&mut self.private_health_number.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("communication_preference:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.communication_preference.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(
                                                &mut self.communication_preference.1,
                                            )
                                            .hint_text("communication_preference"),
                                        );
                                    });
                                    ui.checkbox(&mut self.communication_preference.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("ndis_plan_number:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.ndis_plan_number.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.ndis_plan_number.1)
                                                .hint_text("ndis_plan_number"),
                                        );
                                    });
                                    ui.checkbox(&mut self.ndis_plan_number.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("ndis_plan_start_date:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.ndis_plan_start_date.0, |ui| {
                                        ui.add(
                                            DatePickerButton::new(&mut self.ndis_plan_start_date.1)
                                                .format("%d-%m-%Y")
                                                .highlight_weekends(false)
                                                .id_source("ndis_start"),
                                        );
                                    });
                                    ui.checkbox(&mut self.ndis_plan_start_date.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("ndis_plan_end_date:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.ndis_plan_end_date.0, |ui| {
                                        ui.add(
                                            DatePickerButton::new(&mut self.ndis_plan_end_date.1)
                                                .format("%d-%m-%Y")
                                                .highlight_weekends(false)
                                                .id_source("ndis_end"),
                                        );
                                    });
                                    ui.checkbox(&mut self.ndis_plan_end_date.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("core_funding:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.core_funding.1, "core_funding?");
                                    ui.checkbox(&mut self.core_funding.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("capacity_building_funding:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(
                                        &mut self.capacity_building_funding.1,
                                        "capacity_building_funding?",
                                    );
                                    ui.checkbox(&mut self.capacity_building_funding.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("self_managed:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.self_managed.1, "self_managed?");
                                    ui.checkbox(&mut self.self_managed.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("plan_managed:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.plan_managed.1, "plan_managed?");
                                    ui.checkbox(&mut self.plan_managed.0, "Null?");
                                });

                                ui.end_row();
                            });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("✔ Confirm").clicked() {
                            let edited_participant = Participant {
                                // todo: need to add data validation.
                                id: self.participant_check.id,
                                first_name: self.first_name.clone(),
                                last_name: self.last_name.clone(),
                                medicare_number: self.medicare.clone(),
                                dob: (!self.dob.0).then_some(self.dob.1),
                                address: (!self.address.0).then(|| self.address.1.clone()),
                                suburb: (!self.suburb.0).then(|| self.suburb.1.clone()),
                                postcode: (!self.postcode.0).then(|| self.postcode.1.clone()),
                                phone: (!self.phone.0).then(|| self.phone.1.clone()),
                                email: (!self.email.0).then(|| self.email.1.clone()),
                                medical_notes: (!self.medical_notes.0)
                                    .then(|| self.medical_notes.1.clone()),
                                dietary_notes: (!self.dietary_notes.0)
                                    .then(|| self.dietary_notes.1.clone()),
                                physical_notes: (!self.physical_notes.0)
                                    .then(|| self.physical_notes.1.clone()),
                                other_notes: (!self.other_notes.0)
                                    .then(|| self.other_notes.1.clone()),
                                support_ratio: (!self.support_ratio.0)
                                    .then(|| self.support_ratio.1.clone()),
                                photo_permission: (!self.photo_permission.0)
                                    .then_some(self.photo_permission.1),
                                private_hospital_preference: (!self.private_hospital_preference.0)
                                    .then_some(self.private_hospital_preference.1),
                                private_health_insurer: (!self.private_health_insurer.0)
                                    .then(|| self.private_health_insurer.1.clone()),
                                private_health_number: (!self.private_health_number.0)
                                    .then(|| self.private_health_number.1.clone()),
                                communication_preference: (!self.communication_preference.0)
                                    .then(|| self.communication_preference.1.clone()),
                                ndis_plan_number: (!self.ndis_plan_number.0)
                                    .then(|| self.ndis_plan_number.1.clone()),
                                ndis_plan_start_date: (!self.ndis_plan_start_date.0)
                                    .then_some(self.ndis_plan_start_date.1),
                                core_funding: (!self.core_funding.0).then_some(self.core_funding.1),
                                capacity_building_funding: (!self.capacity_building_funding.0)
                                    .then_some(self.capacity_building_funding.1),
                                self_managed: (!self.self_managed.0).then_some(self.self_managed.1),
                                plan_managed: (!self.plan_managed.0).then_some(self.plan_managed.1),
                                ndis_plan_end_date: (!self.ndis_plan_end_date.0)
                                    .then_some(self.ndis_plan_end_date.1),
                            };

                            self.db.edit_participant(edited_participant).unwrap();
                            self.participant_check.id = None;
                            self.changed = true;
                        };
                        if ui.button("❌ Delete").clicked() {
                            // todo: need to add confirmation button.
                            self.db.delete_participant(self.participant_check.id.unwrap()).unwrap();
                            self.participant_check.id = None;
                            self.changed = true;
                        };
                    });
                }
            });
    }
}
