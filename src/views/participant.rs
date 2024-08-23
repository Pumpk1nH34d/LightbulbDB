use egui::{Context, Ui};
use chrono::NaiveDate;
use crate::database_logic::data_structs::Participant;
use crate::windows::participant::{edit_participant_window::EditWindow, add_participant_window::AddWindow};
use crate::database_logic::database::DataBase;

#[derive(Default)]
pub struct ParticipantsView {
    db: DataBase,
    search_response: String,
    add_window: AddWindow,
    edit_window: EditWindow,

    selected_participant: Participant
}

impl ParticipantsView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let mut stmt = self.db.connection.prepare("SELECT * FROM Participants").unwrap();
        let participants = stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                medicare_number: row.get_unwrap(3),
                dob: Some(row.get_unwrap::<_, String>(4).parse::<NaiveDate>().unwrap_or_default()),
                address: row.get(5).unwrap_or(Some(String::new())),
                suburb: row.get(6).unwrap_or(Some(String::new())),
                postcode: row.get(7).unwrap_or(Some(String::new())),
                phone: row.get(8).unwrap_or(Some(String::new())),
                email: row.get(9).unwrap_or(Some(String::new())),
                medical_notes: row.get(10).unwrap_or(Some(String::new())),
                dietary_notes: row.get(11).unwrap_or(Some(String::new())),
                physical_notes: row.get(12).unwrap_or(Some(String::new())),
                other_notes: row.get(13).unwrap_or(Some(String::new())),
                support_ratio: row.get(14).unwrap_or(Some(String::new())),
                photo_permission: row.get(15).unwrap_or(Some(false)),
                private_hospital_preference: row.get(16).unwrap_or(Some(false)),
                private_health_insurancer: row.get(17).unwrap_or(Some(String::new())),
                private_health_number: row.get(18).unwrap_or(Some(String::new())),
                communication_preference: row.get(19).unwrap_or(Some(String::new())),
                ndis_plan_number: row.get(20).unwrap_or(Some(String::new())),
                ndis_plan_start_date: Some(row.get_unwrap::<_, String>(21).parse::<NaiveDate>().unwrap_or_default()),
                core_funding: row.get(22).unwrap_or(Some(false)),
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),
                self_managed: row.get(24).unwrap_or(Some(false)),
                plan_managed: row.get(25).unwrap_or(Some(false)),
                ndis_plan_end_date: Some(row.get_unwrap::<_, String>(26).parse::<NaiveDate>().unwrap_or_default()),
            })
        }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();
        let size = participants.len();
        ui.vertical_centered(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search_response)
                    .hint_text("🔍 Type to search..."),
            );
        });
        ui.separator();egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Date of Birth");
                ui.label("Phone Number");
                ui.label("Email");
                ui.label("Support Ratio");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for index in 0..size {
                        if ui.button(format!("{} {}", &participants[index].first_name, &participants[index].last_name)).clicked() {
                            self.selected_participant = participants[index].clone();
                        }
                        ui.label(&participants[index].dob.unwrap().format("%d-%m-%Y").to_string());
                        ui.label(&participants[index].phone.clone().unwrap());
                        ui.label(&participants[index].email.clone().unwrap());
                        ui.label(format!("1:{}", &participants[index].support_ratio.clone().unwrap()));
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
                ui.vertical_centered(|ui| {
                    ui.heading(format!("{} {}", self.selected_participant.first_name, self.selected_participant.last_name));
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(format!("{} {}", self.selected_participant.first_name, self.selected_participant.last_name));
                });
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
        self.edit_window.ui(ui, ctx);
    }
}


