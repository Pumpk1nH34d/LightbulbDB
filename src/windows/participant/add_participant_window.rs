use crate::database_logic::data_structs::Participant;
use chrono::NaiveDate;
use egui::{Context, TextEdit, Ui};
use egui_extras::DatePickerButton;
use crate::database_logic::database::DataBase;

#[derive(Default)]
pub struct AddWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    first_name: String,
    last_name: String,
    dob: (bool, NaiveDate),
    phone: (bool, String),
    email: (bool, String),
    medicare: String,
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

impl AddWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Create Participant")
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("my_grid")
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
                                    ui.add(DatePickerButton::new(&mut self.dob.1).highlight_weekends(false));
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
                                    ui.add(TextEdit::singleline(&mut self.email.1).hint_text("Email"));
                                });
                                ui.checkbox(&mut self.email.0, "Null?");
                            });
                            ui.end_row();
                            ui.label("Medicare Number:");
                            ui.add(TextEdit::singleline(&mut self.medicare).hint_text("Medicare"));
                            ui.end_row();
                            ui.label("medical_notes:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.medical_notes.0, |ui| {
                                    ui.add(TextEdit::singleline(&mut self.medical_notes.1).hint_text("medical_notes"));
                                });
                                ui.checkbox(&mut self.medical_notes.0, "Null?");
                            });
                            ui.end_row();
                            ui.label("dietary_notes:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.dietary_notes.0, |ui| {
                                    ui.add(TextEdit::singleline(&mut self.dietary_notes.1).hint_text("dietary_notes"));
                                });
                                ui.checkbox(&mut self.dietary_notes.0, "Null?");
                            });
                            ui.end_row();
                            ui.label("physical_notes:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.physical_notes.0, |ui| {
                                    ui.add(TextEdit::singleline(&mut self.physical_notes.1).hint_text("physical_notes"));
                                });
                                ui.checkbox(&mut self.physical_notes.0, "Null?");
                            });
                            ui.end_row();
                            ui.label("other_notes:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.other_notes.0, |ui| {
                                    ui.add(TextEdit::singleline(&mut self.other_notes.1).hint_text("other_notes"));
                                });
                                ui.checkbox(&mut self.other_notes.0, "Null?");
                            });
                            ui.end_row();
                            ui.label("support_ratio:");
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(!self.support_ratio.0, |ui| {
                                    ui.add(TextEdit::singleline(&mut self.support_ratio.1).hint_text("support_ratio"));
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

                        });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("➕ ADD").clicked() {
                        let new_participant = Participant {
                            first_name: self.first_name.clone(),
                            last_name: self.last_name.clone(),
                            medicare_number: self.medicare.clone(),
                            dob: self.dob.0.then_some(self.dob.1),
                            address: self.address.0.then(|| self.address.1.clone()),
                            suburb: self.suburb.0.then(|| self.suburb.1.clone()),
                            postcode: self.postcode.0.then(|| self.postcode.1.clone()),
                            phone: self.phone.0.then(|| self.phone.1.clone()),
                            email: self.email.0.then(|| self.email.1.clone()),
                            medical_notes: self.medical_notes.0.then(|| self.medical_notes.1.clone()),
                            dietary_notes: self.physical_notes.0.then(|| self.physical_notes.1.clone()),
                            physical_notes: self.physical_notes.0.then(|| self.physical_notes.1.clone()),
                            other_notes: self.other_notes.0.then(|| self.other_notes.1.clone()),
                            support_ratio: self.support_ratio.0.then(|| self.support_ratio.1.clone()),
                            photo_permission: self.photo_permission.0.then_some(self.photo_permission.1),
                            private_hospital_preference: self.private_hospital_preference.0.then_some(self.private_hospital_preference.1),
                            private_health_insurancer: self.private_health_insurer.0.then(|| self.private_health_insurer.1.clone()),
                            private_health_number: self.private_health_number.0.then(|| self.private_health_number.1.clone()),
                            communication_preference: self.communication_preference.0.then(|| self.communication_preference.1.clone()),
                            ndis_plan_number: self.ndis_plan_number.0.then(|| self.ndis_plan_number.1.clone()),
                            ndis_plan_start_date: self.ndis_plan_start_date.0.then_some(self.ndis_plan_start_date.1),
                            core_funding: self.core_funding.0.then_some(self.core_funding.1),
                            capacity_building_funding: self.capacity_building_funding.0.then_some(self.capacity_building_funding.1),
                            self_managed: self.self_managed.0.then_some(self.self_managed.1),
                            plan_managed: self.plan_managed.0.then_some(self.plan_managed.1),
                            ndis_plan_end_date: self.ndis_plan_end_date.0.then_some(self.ndis_plan_end_date.1),
                        };
                        self.db.create_participant(new_participant).unwrap();
                    };
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;
                    };
                });
            });
        if !self.open | self.reset {
            self.reset_values();
        }
    }

    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
