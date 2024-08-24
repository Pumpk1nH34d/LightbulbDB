use crate::database_logic::data_structs::Venue;
use crate::database_logic::database::DataBase;
use crate::windows::venue::{
    add_venue_window::AddWindow, edit_venue_window::EditWindow,
};
use egui::{Context, Ui};

#[derive(Default)]
pub struct VenuesView {
    db: DataBase,
    search_response: String,
    add_window: AddWindow,
    edit_window: EditWindow,

    selected_venue: Venue,
}

impl VenuesView {
    pub fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        self.right_panel_view(ui);
        self.bottom_menu_view(ui);
        self.load_windows_ui(ui, ctx);
        self.main_view(ui);
    }

    fn main_view(&mut self, ui: &mut Ui) {
        let venues = self.db.get_all_venues();
        let size = venues.len();
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
                ui.label("Address");
                ui.label("Postcode");
                ui.label("State");
                ui.label("Price");
                ui.end_row();
            });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("venue_results")
                .num_columns(5)
                .spacing([30.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for index in 0..size {
                        if ui
                            .button(format!(
                                "{}",
                                &venues[index].name,
                            ))
                            .clicked()
                        {
                            self.selected_venue = venues[index].clone();
                        }
                        ui.label(&venues[index].address.clone().unwrap());
                        ui.label(&venues[index].postcode.clone().unwrap());
                        ui.label(&venues[index].state.clone().unwrap());
                        ui.label(&venues[index].price.clone().unwrap());
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
                if self.selected_venue.id.is_some() {
                    ui.vertical_centered(|ui| {
                        ui.heading(format!(
                            "{}",
                            self.selected_venue.name
                        ));
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(format!(
                            "Name: {}",
                            self.selected_venue.name
                        ));
                        ui.label(format!(
                            "address: {}",
                            self.selected_venue
                                .address.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "suburb: {}",
                            self.selected_venue
                                .suburb.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "postcode: {}",
                            self.selected_venue
                                .postcode.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "state: {}",
                            self.selected_venue
                                .state.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "description: {}",
                            self.selected_venue
                                .description.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "contact_person_name: {}",
                            self.selected_venue
                                .contact_person_name.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "contact_person_phone: {}",
                            self.selected_venue
                                .contact_person_phone.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "venue_phone_number: {}",
                            self.selected_venue
                                .venue_phone_number.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "price: {}",
                            self.selected_venue
                                .price.clone()
                                .unwrap_or_default()
                        ));
                        ui.label(format!(
                            "notes: {}",
                            self.selected_venue
                                .notes.clone()
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
            .ui(ui, ctx, self.selected_venue.clone());
    }
}
