#[derive(Default)]
pub struct LineItemsView {}

impl LineItemsView {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Support Workers");
    }
}
