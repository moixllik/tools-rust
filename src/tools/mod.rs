pub mod calculator;
pub mod speecher;

use eframe::egui;

pub trait Tool {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}