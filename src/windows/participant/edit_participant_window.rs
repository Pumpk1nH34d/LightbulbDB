use egui::{Context, Ui};

pub struct EditWindow {
    pub open: bool,
}

impl EditWindow {
    pub fn ui(&mut self, _ui: &mut Ui, ctx: &Context) {
        egui::Window::new("Edit")
            .open(&mut self.open)
            .show(ctx, |ui| {
                ui.label("LOL");
            });
    }

    pub fn reset(&mut self) {
        self.open = false;
    }
}

impl Default for EditWindow {
    fn default() -> Self {
        EditWindow {
            open: false,
        }
    }
}