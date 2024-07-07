#[derive(Default)]
pub struct SupportWorkersView {}

impl SupportWorkersView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Support Workers");
    }
}
