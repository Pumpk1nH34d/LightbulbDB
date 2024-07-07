#[derive(Default)]
pub struct WorkshopsView {}

impl WorkshopsView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Workshops");
    }
}
