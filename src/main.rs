mod database_logic;
mod views;
mod windows;

// Import necessary modules and structs from views and database_logic
use crate::views::{
    participant::ParticipantsView, support_worker::SupportWorkersView, venue::VenuesView,
    workshop::WorkshopsView,
};
use eframe::egui;
use egui::Ui;
use crate::database_logic::database::DataBase;

// Define an enum to represent the different views available in the app
#[derive(PartialEq, Default)]
enum Views {
    #[default] // Set ParticipantsView as the default view
    ParticipantsView,
    Workshops,
    SupportWorkers,
    Venues,
}

// Define a struct to hold the app's data and current state
#[derive(Default)]
struct Content {
    db: DataBase, // Database instance
    current_view: Views, // Currently selected view
    participants: ParticipantsView, // Participants view
    workshops: WorkshopsView, // Workshops view
    support_workers: SupportWorkersView, // Support workers view
    venues: VenuesView, // Venues view
}

// Implement the eframe::App trait for the Content struct
impl eframe::App for Content {
    // Define the update method to render the UI
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.top_menu(ui); // Render the top menu
            ui.separator(); // Add a separator
            self.main_view(ui, ctx); // Render the main view based on the current view
        });
    }
}

// Implement methods for the Content struct
impl Content {
    // Render the top menu with selectable views and a populate button
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

            // Button to populate the database
            if ui.button("POPULATE").clicked() {
                self.db.drop_db().unwrap(); // Drop the existing database
                self.db.create_db().unwrap(); // Create a new database
                self.db.populate_database().unwrap(); // Populate the database with initial data
            }
        });
    }

    // Render the main view based on the current view selected in the top menu
    fn main_view(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        match self.current_view {
            Views::ParticipantsView => self.participants.ui(ui, ctx), // Render participants view
            Views::Workshops => self.workshops.ui(ui, ctx), // Render workshops view
            Views::SupportWorkers => self.support_workers.ui(ui, ctx), // Render support workers view
            Views::Venues => self.venues.ui(ui, ctx), // Render venues view
        }
    }
}

// Entry point of the application
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default(); // Set default options for the native window
    eframe::run_native(
        "LightBulbDB", // Application title
        options, // Native options
        Box::new(|_cc| Ok(Box::<Content>::default())), // Initialize the app with default Content
    )
}
