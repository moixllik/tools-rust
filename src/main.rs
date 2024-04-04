#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod calculator;

use calculator::Calculator;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tools",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

#[derive(Default)]
struct MyApp {
    calculator: Calculator,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
            ui.menu_button("Tools", |ui| {
                if ui.button("Calculator").clicked() {
                    self.calculator.is_opened = true;
                    ui.close_menu()
                }
            });
        });

        self.calculator.show(ctx);
    }
}
