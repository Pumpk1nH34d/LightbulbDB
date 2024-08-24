use crate::database_logic::database::DataBase;
use egui::{Context, TextEdit, Ui};

#[derive(Default)]
pub struct FilterWindow {
    pub open: bool,
    pub db: DataBase,
    reset: bool,

    filter: String,

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

impl FilterWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) -> String {
        egui::Window::new("Filer Venue")
            .open(&mut self.open)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("filter_venue_grid")
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
                            ui.label("address:");
                            ui.add(
                                TextEdit::singleline(&mut self.address)
                                    .hint_text("address"),
                            );
                            ui.end_row();
                            ui.label("suburb:");
                            ui.add(
                                TextEdit::singleline(&mut self.suburb)
                                    .hint_text("suburb"),
                            );
                            ui.end_row();
                            ui.label("postcode:");
                            ui.add(
                                TextEdit::singleline(&mut self.postcode)
                                    .hint_text("postcode"),
                            );
                            ui.end_row();
                            ui.label("other_qualifications:");
                            ui.add(
                                TextEdit::singleline(&mut self.other_qualifications)
                                    .hint_text("other_qualifications"),
                            );
                            ui.end_row();
                            ui.label("notes:");
                            ui.add(
                                TextEdit::singleline(&mut self.notes)
                                    .hint_text("other_qualifications"),
                            );
                            ui.end_row();
                        });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("✔ APPLY").clicked() {
                        let mut filter = String::new();
                        if !self.first_name.is_empty() {
                            filter += &format!("first_name = '{}', ", self.first_name)
                        };
                        if !self.last_name.is_empty() {
                            filter += &format!("last_name = '{}', ", self.last_name)
                        }
                        if !self.phone.is_empty() {
                            filter += &format!("phone = '{}', ", self.phone)
                        }
                        if !self.email.is_empty() {
                            filter += &format!("email = '{}', ", self.email)
                        }
                        if !self.address.is_empty() {
                            filter += &format!("address = '{}', ", self.address)
                        }
                        if !self.suburb.is_empty() {
                            filter += &format!("suburb = '{}', ", self.suburb)
                        }
                        if !self.postcode.is_empty() {
                            filter += &format!("postcode = '{}', ", self.postcode)
                        }
                        if !self.other_qualifications.is_empty() {
                            filter += &format!("other_qualifications = '{}', ", self.other_qualifications)
                        }
                        if !self.notes.is_empty() {
                            filter += &format!("notes = '{}', ", self.notes)
                        }
                        if !filter.is_empty() {
                            filter.truncate(filter.len() - 2)
                        }
                        self.filter = filter;
                    }
                    if ui.button("🔃 Reset").clicked() {
                        self.reset = true;
                    };
                });
            });
        if self.reset {
            self.reset_values();
        };
        self.filter.clone()
    }

    pub fn reset_values(&mut self) {
        (*self, self.open) = (Self::default(), self.open);
    }
}
