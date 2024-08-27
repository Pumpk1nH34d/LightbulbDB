mod database_logic;
mod views;
mod windows;

use crate::views::{
    participant::ParticipantsView, support_worker::SupportWorkersView, venue::VenuesView,
    workshop::WorkshopsView,
};
use eframe::egui;
use egui::Ui;
use crate::database_logic::database::DataBase;

//todo: comment code

#[derive(PartialEq, Default)]
enum Views {
    #[default]
    ParticipantsView,
    Workshops,
    SupportWorkers,
    Venues,
}

#[derive(Default)]
struct Content {
    db: DataBase,
    current_view: Views,
    participants: ParticipantsView,
    workshops: WorkshopsView,
    support_workers: SupportWorkersView,
    venues: VenuesView,
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.top_menu(ui);
            ui.separator();
            self.main_view(ui, ctx);
        });
    }
}

impl Content {
    fn top_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.current_view,
                Views::ParticipantsView,
                "👱 Participants",
            );
            ui.selectable_value(&mut self.current_view, Views::Workshops, "🛠 Workshops");
            ui.selectable_value(
                &mut self.current_view,
                Views::SupportWorkers,
                "📖 Support Workers",
            );
            ui.selectable_value(&mut self.current_view, Views::Venues, "🏡 Venues");
            if ui.button("POPULATE").clicked() {
                self.db.drop_db().unwrap();
                self.db.create_db().unwrap();
                self.db.populate_database().unwrap();
            }
        });
    }

    fn main_view(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        match self.current_view {
            Views::ParticipantsView => self.participants.ui(ui, ctx),
            Views::Workshops => self.workshops.ui(ui, ctx),
            Views::SupportWorkers => self.support_workers.ui(ui, ctx),
            Views::Venues => self.venues.ui(ui, ctx),
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "LightBulbDB",
        options,
        Box::new(|_cc| Ok(Box::<Content>::default())),
    )
}
