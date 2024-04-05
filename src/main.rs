#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod tools;

use eframe::egui;
use tools::{calculator::Calculator, speecher::Speecher, Tool};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tools",
        native_options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    open: Vec<bool>,
    calculator: Calculator,
    speecher: Speecher,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut speecher = Speecher::default();
        speecher.voices = tools::speecher::voices_list();

        Self {
            open: vec![false; 2],
            calculator: Calculator::default(),
            speecher: speecher,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::global_dark_light_mode_switch(ui);

                ui.separator();

                ui.menu_button("Math", |ui| {
                    if ui.button("Calculator").clicked() {
                        self.open[0] = true;
                        ui.close_menu();
                    }
                });

                ui.separator();

                ui.menu_button("Audio", |ui| {
                    if ui.button("Speecher").clicked() {
                        self.open[1] = true;
                        ui.close_menu();
                    }
                });
            });
        });

        self.calculator.show(ctx, &mut self.open[0]);
        self.speecher.show(ctx, &mut self.open[1]);
    }
}
