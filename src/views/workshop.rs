use crate::database_logic::data_structs::Workshop;
use crate::database_logic::database::DataBase;
use crate::windows::workshop::{add_workshop_window::AddWindow, edit_workshop_window::EditWindow, filter_workshop_window::FilterWindow};
use egui::{Context, Ui};

#[derive(Default)]
pub struct WorkshopsView {
    db: DataBase,

    filter: String,
    name_filter: String,
    add_window: AddWindow,
    edit_window: EditWindow,
    filter_window: FilterWindow,

    selected_workshop: Workshop,
}

impl WorkshopsView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let workshops = if self.filter.is_empty() {
            self.db.get_all_workshops()
        } else {
            self.db.get_filtered_workshops(self.filter.clone())
        };
        let size = workshops.len();
        egui::Grid::new("headings")
            .num_columns(5)
            .spacing([30.0, 4.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Facilitator");
                ui.label("Venue");
                ui.label("Start Date");
                ui.label("End Date");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("venue_results")
                .num_columns(6)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for index in 0..size {
                        if ui.button(format!("{}", &workshops[index].name,)).clicked() {
                            self.selected_workshop = workshops[index].clone();
                        }
                        ui.label(&workshops[index].name.clone());
                        ui.label(&workshops[index].facilitator.clone().to_string());
                        ui.label(&workshops[index].venue.clone().to_string());
                        ui.label(&workshops[index].start_date.clone().to_string());
                        ui.label(&workshops[index].end_date.clone().to_string());
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
                if self.selected_workshop.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(format!("{}", self.selected_workshop.name));
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!("Name: {}", self.selected_workshop.name));
                        ui.label(format!(
                            "Facilitator: {}",
                            self.selected_workshop.facilitator.clone()
                        ));
                        ui.label(format!(
                            "Venue: {}",
                            self.selected_workshop.venue.clone()
                        ));
                        ui.label(format!(
                            "start_date: {}",
                            self.selected_workshop.start_date.clone()
                        ));
                        ui.label(format!(
                            "end_date: {}",
                            self.selected_workshop.end_date.clone()
                        ));
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("SELECT WORKSHOP");
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
                    if ui.button("⛭ Filter").clicked() {
                        self.filter_window.open = !self.filter_window.open;
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
        self.edit_window.ui(ui, ctx, self.selected_workshop.clone());
        self.filter = self.filter_window.ui(ui, ctx);
    }
}
