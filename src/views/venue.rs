#[derive(Default)]
pub struct VenuesView {}

impl VenuesView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Venues");
    }
}
