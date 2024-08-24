use crate::database_logic::data_structs::{SupportWorker};
use crate::database_logic::database::DataBase;
use crate::windows::support_workers::{
    add_support_worker_window::AddWindow, edit_support_worker_window::EditWindow,
};
use egui::{Context, Ui};

#[derive(Default)]
pub struct SupportWorkersView {
    db: DataBase,
    search_response: String,
    add_window: AddWindow,
    edit_window: EditWindow,

    selected_support_worker: SupportWorker,
}

impl SupportWorkersView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let support_workers = self.db.get_all_support_workers();
        let size = support_workers.len();
        ui.vertical_centered(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search_response)
                    .hint_text("🔍 Type to search..."),
            );
        });
        ui.separator();
        egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Date of Birth");
                ui.label("Phone Number");
                ui.label("Email");
                ui.label("Car Insurance");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for index in 0..size {
                        if ui
                            .button(format!(
                                "{} {}",
                                &support_workers[index].first_name,
                                &support_workers[index].last_name
                            ))
                            .clicked()
                        {
                            self.selected_support_worker = support_workers[index].clone();
                        }
                        ui.label(match support_workers[index].dob {
                            None => String::new(),
                            Some(value) => value.format("%d-%m-%Y").to_string(),
                        });
                        ui.label(&support_workers[index].phone);
                        ui.label(&support_workers[index].email);
                        ui.label(match &support_workers[index].car_insurance {
                            None => String::new(),
                            Some(value) => {
                                value.to_string()
                            }
                        });
                        ui.end_row();
                    }
                });
        });
    }
    fn right_panel_view(&mut self, ui: &mut Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                if self.selected_support_worker.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(format!(
                            "{} {}",
                            self.selected_support_worker.first_name,
                            self.selected_support_worker.last_name
                        ));
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!(
                            "Name: {} {}",
                            self.selected_support_worker.first_name,
                            self.selected_support_worker.last_name
                        ));
                        ui.label(format!(
                            "dob: {}",
                            self.selected_support_worker
                                .dob
                                .unwrap_or_default()
                                .to_string()
                        ));
                        ui.label(format!(
                            "phone: {}",
                            self.selected_support_worker
                                .phone
                        ));
                        ui.label(format!(
                            "email: {}",
                            self.selected_support_worker
                                .email
                        ));
                        ui.label(format!(
                            "address: {}",
                            self.selected_support_worker
                                .address
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "suburb: {}",
                            self.selected_support_worker.suburb.clone().unwrap_or_default()
                        ));
                        ui.label(format!(
                            "postcode: {}",
                            self.selected_support_worker
                                .postcode
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "first_aid: {}",
                            self.selected_support_worker
                                .first_aid
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "confidentiality_agreement: {}",
                            self.selected_support_worker
                                .confidentiality_agreement
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "police_clearance: {}",
                            self.selected_support_worker
                                .police_clearance
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "car_insurance: {}",
                            self.selected_support_worker
                                .car_insurance
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "other_qualifications: {}",
                            self.selected_support_worker
                                .other_qualifications
                                .clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "notes: {}",
                            self.selected_support_worker
                                .notes
                                .clone()
                                .unwrap_or_default()
                        ));

                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT SUPPORT WORKER");
                    });
                }
            });
    }

    fn bottom_menu_view(&mut self, ui: &mut Ui) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .max_height(25.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("➕ Create").clicked() {
                        self.add_window.open = !self.add_window.open;
                    };
                    if ui.button("✏ Edit").clicked() {
                        self.edit_window.open = !self.edit_window.open;
                    };
                    if ui.button("RESET DB").clicked() {
                        self.db.drop_db().unwrap();
                        self.db.create_db().unwrap();
                    };
                });
            });
    }
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ui, ctx);
        self.edit_window
            .ui(ui, ctx, self.selected_support_worker.clone());
    }
}
