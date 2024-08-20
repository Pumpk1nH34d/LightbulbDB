use egui::{Context, Ui};

// use crate::database_logic::data_structs::Participant;
use crate::windows::participant::{edit_window::EditWindow, add_participant_window::AddWindow};
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
        ui.vertical_centered(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.search_response)
                    .hint_text("🔍 Type to search..."),
            );
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
                        self.db.drop_db().unwrap();
                        self.db.create_db().unwrap();
                        self.add_window.open = !self.add_window.open;
                    };
                    if ui.button("✏ Edit").clicked() {
                        self.edit_window.open = !self.edit_window.open;
                    };
                });
            });
    }
    fn load_windows_ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.add_window.ui(ui, ctx);
        self.edit_window.ui(ui, ctx);
    }
}


