use egui::Ui;
use crate::database::{create_db, drop_db};
use rusqlite::{Connection, Result};
enum View {
    Read,
    Add,
    Edit,
}

#[derive(Default)]
pub struct ParticipantsView {
}

impl ParticipantsView {
    pub fn ui(&mut self, ui: &mut egui::Ui){
        
        ui.label("TEST");
        self.right_panel(ui);
    }
    
    fn right_panel(&mut self, ui: &mut egui::Ui) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Austin Delic");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Right Panel!");
                });
            });
    }
}
