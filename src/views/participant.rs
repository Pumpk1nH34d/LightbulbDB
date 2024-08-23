use egui::{Context, Ui};
use crate::windows::participant::{edit_participant_window::EditWindow, add_participant_window::AddWindow};
use crate::database_logic::database::DataBase;

#[derive(Default)]
pub struct ParticipantsView {
    db: DataBase,
    search_response: String,
    add_window: AddWindow,
    edit_window: EditWindow,
}

impl ParticipantsView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui, ctx);
    }

    fn main_view(&mut self, ui: &mut Ui, ctx: &Context) {
        let mut stmt = self.db.connection.prepare("SELECT first_name, last_name, medicare_number, dob, postcode, phone, email FROM Participants").unwrap();
        let mut rows = stmt.query([]).unwrap();

        let mut first_names:Vec<String> = Vec::new();
        let mut last_names:Vec<String> = Vec::new();
        let mut medicare_numbers:Vec<String> = Vec::new();
        let mut dobs:Vec<String> = Vec::new();
        let mut postcodes:Vec<String> = Vec::new();
        let mut phones:Vec<String> = Vec::new();
        let mut emails:Vec<String> = Vec::new();

        while let Some(row) = rows.next().unwrap() {
            first_names.push(row.get(0).unwrap());
            last_names.push(row.get(1).unwrap());
            medicare_numbers.push(row.get(2).unwrap_or(String::from("")));
            dobs.push(row.get(3).unwrap_or(String::from("")));
            postcodes.push(row.get(4).unwrap_or(String::from("")));
            phones.push(row.get(5).unwrap_or(String::from("")));
            emails.push(row.get(6).unwrap_or(String::from("")));
        }
        let size = first_names.len();
        let mut checkbox_bool = vec!(false; size);
        ui.vertical_centered(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search_response)
                    .hint_text("🔍 Type to search..."),
            );
        });
        ui.separator();
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("headings")
                .num_columns(7)
                .spacing([30.0, 4.0])
                .striped(false)
                .show(ui, |ui| {
                    ui.end_row();
                    ui.label("First Name");
                    ui.label("Last Name");
                    ui.label("Medicare Number");
                    ui.label("DOB");
                    ui.label("Postcode");
                    ui.label("Phone Number");
                    ui.label("Email");
                    ui.end_row();
                });
            egui::Grid::new("results")
                .num_columns(8)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for index in 0..size {
                        ui.label(&first_names[index]);
                        ui.label(&last_names[index]);
                        ui.label(&medicare_numbers[index]);
                        ui.label(&dobs[index]);
                        ui.label(&postcodes[index]);
                        ui.label(&phones[index]);
                        ui.label(&emails[index]);
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
                    ui.heading(&self.search_response);
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Right Panel!");
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


