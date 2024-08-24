use crate::database_logic::data_structs::Venue;
use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};

#[derive(Default)]
pub struct EditWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    changed: bool,
    venue_check: Venue,

    name: String,
    address: (bool, String),
    suburb: (bool, String),
    postcode: (bool, String),
    state: (bool, String),
    description: (bool, String),
    contact_person_name: (bool, String),
    contact_person_phone: (bool, String),
    venue_phone_number: (bool, String),
    price: (bool, String),
    notes: (bool, String),
}

impl EditWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context, venue: Venue) {
        if self.venue_check.id != venue.id {
            self.venue_check = venue.clone();
            self.name = venue.name;
            match venue.address {
                None => self.address.0 = true,
                Some(value) => self.address = (false, value),
            }
            match venue.suburb {
                None => self.suburb.0 = true,
                Some(value) => self.suburb = (false, value),
            }
            match venue.postcode {
                None => self.postcode.0 = true,
                Some(value) => self.postcode = (false, value),
            }
            match venue.state {
                None => self.state.0 = true,
                Some(value) => self.state = (false, value),
            }
            match venue.description {
                None => self.description.0 = true,
                Some(value) => self.description = (false, value),
            }
            match venue.contact_person_name {
                None => self.contact_person_name.0 = true,
                Some(value) => self.contact_person_name = (false, value),
            }
            match venue.contact_person_phone {
                None => self.contact_person_phone.0 = true,
                Some(value) => self.contact_person_phone = (false, value),
            }
            match venue.venue_phone_number {
                None => self.venue_phone_number.0 = true,
                Some(value) => self.venue_phone_number = (false, value),
            }
            match venue.price {
                None => self.price.0 = true,
                Some(value) => self.price = (false, value),
            }
            match venue.notes {
                None => self.notes.0 = true,
                Some(value) => self.notes = (false, value),
            }
        }
        if !self.open {
            self.changed = false;
        };
        egui::Window::new("Edit Venue")
            .open(&mut self.open)
            .show(ctx, |ui| {
                if self.changed {
                    ui.label("DONE ✔");
                    ui.label("Reselect Venue to see changes.");
                } else if self.venue_check.id.is_none() {
                    ui.label("Select a Venue to EDIT");
                } else {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("edit_venue")
                            .num_columns(2)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Name:");
                                ui.add(TextEdit::singleline(&mut self.name).hint_text("name"));
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
                                ui.label("State:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.state.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.state.1)
                                                .hint_text("postcode"),
                                        );
                                    });
                                    ui.checkbox(&mut self.state.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("description:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.description.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.description.1)
                                                .hint_text("description"),
                                        );
                                    });
                                    ui.checkbox(&mut self.description.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("contact_person_name:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.contact_person_name.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.contact_person_name.1)
                                                .hint_text("contact_person_name"),
                                        );
                                    });
                                    ui.checkbox(&mut self.contact_person_name.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("contact_person_phone:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.contact_person_phone.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.contact_person_phone.1)
                                                .hint_text("contact_person_phone"),
                                        );
                                    });
                                    ui.checkbox(&mut self.contact_person_phone.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("venue_phone_number:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.venue_phone_number.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.venue_phone_number.1)
                                                .hint_text("venue_phone_number"),
                                        );
                                    });
                                    ui.checkbox(&mut self.venue_phone_number.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("price:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.price.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.price.1)
                                                .hint_text("price"),
                                        );
                                    });
                                    ui.checkbox(&mut self.price.0, "Null?");
                                });
                                ui.end_row();
                                ui.label("notes:");
                                ui.horizontal(|ui| {
                                    ui.add_enabled_ui(!self.notes.0, |ui| {
                                        ui.add(
                                            TextEdit::singleline(&mut self.notes.1)
                                                .hint_text("notes"),
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
                            let edited_venue = Venue {
                                id: self.venue_check.id,
                                name: self.name.clone(),
                                address: (!self.address.0).then(|| self.address.1.clone()),
                                suburb: (!self.suburb.0).then(|| self.suburb.1.clone()),
                                postcode: (!self.postcode.0).then(|| self.postcode.1.clone()),
                                state: (!self.state.0).then(|| self.state.1.clone()),
                                description: (!self.description.0)
                                    .then(|| self.description.1.clone()),
                                contact_person_name: (!self.contact_person_name.0)
                                    .then(|| self.contact_person_name.1.clone()),
                                contact_person_phone: (!self.contact_person_phone.0)
                                    .then(|| self.contact_person_phone.1.clone()),
                                venue_phone_number: (!self.venue_phone_number.0)
                                    .then(|| self.venue_phone_number.1.clone()),
                                price: (!self.price.0).then(|| self.price.1.clone()),
                                notes: (!self.notes.0).then(|| self.notes.1.clone()),
                            };
                            self.db.edit_venue(edited_venue).unwrap();
                            self.venue_check.id = None;
                            self.changed = true;
                        };
                        if ui.button("❌ Delete").clicked() {
                            self.db.delete_venues(self.venue_check.id.unwrap());
                            self.venue_check.id = None;
                            self.changed = true;
                        };
                    });
                }
            });
    }
}
