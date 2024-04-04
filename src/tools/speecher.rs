use eframe::egui;

#[derive(Default)]
pub struct Speecher {
    pub is_opened: bool,
}

impl Speecher {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Speecher")
            .resizable(false)
            .open(&mut self.is_opened)
            .show(ctx, |ui| {

            });
    }
}