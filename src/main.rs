use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

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

/* CALCULATOR */
#[derive(Default)]
struct Calculator {
    is_opened: bool,
    text_result: String,
}

impl Calculator {
    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Calculator")
            .resizable(false)
            .open(&mut self.is_opened)
            .show(ctx, |ui| {
                egui::Grid::new("buttons").show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text_result)
                            .horizontal_align(egui::Align::RIGHT)
                            .desired_rows(3),
                    );
                    ui.end_row();
                    
                    let mut limit = 0;
                    for ch in "7 8 9 / 4 5 6 * 1 2 3 - 0 . = +".split(" ") {
                        if ui.button(ch).clicked() {};
                        limit += 1;
                        if limit % 4 == 0 {
                            ui.end_row();
                        }
                    }
                })
            });
    }
}
