use eframe::egui;

#[derive(Default)]
pub struct Calculator {
    pub is_opened: bool,
    text_result: String,
    result: f64,
    back_line: String,
}

impl Calculator {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Calculator")
            .resizable(false)
            .open(&mut self.is_opened)
            .show(ctx, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.text_result)
                        .horizontal_align(egui::Align::RIGHT)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );

                ui.columns(4, |col| {
                    if col[0].button("7").clicked() {
                        self.text_result += "7";
                    }
                    if col[1].button("8").clicked() {
                        self.text_result += "8";
                    }
                    if col[2].button("9").clicked() {
                        self.text_result += "9";
                    }
                    if col[3].button("/").clicked() {
                        self.text_result += "\n/\n";
                    }
                });

                ui.columns(4, |col| {
                    if col[0].button("4").clicked() {
                        self.text_result += "4";
                    }
                    if col[1].button("5").clicked() {
                        self.text_result += "5";
                    }
                    if col[2].button("6").clicked() {
                        self.text_result += "6";
                    }
                    if col[3].button("*").clicked() {
                        self.text_result += "\n*\n";
                    }
                });

                ui.columns(4, |col| {
                    if col[0].button("1").clicked() {
                        self.text_result += "1";
                    }
                    if col[1].button("2").clicked() {
                        self.text_result += "2";
                    }
                    if col[2].button("3").clicked() {
                        self.text_result += "3";
                    }
                    if col[3].button("-").clicked() {
                        self.text_result += "\n-\n";
                    }
                });

                ui.columns(4, |col| {
                    if col[0].button("0").clicked() {
                        self.text_result += "0";
                    }
                    if col[1].button(".").clicked() {
                        self.text_result += ".";
                    }
                    if col[2].button("=").clicked() {
                        for line in self.text_result.clone().split("\n") {
                            if let Ok(num) = line.parse::<f64>() {
                                match self.back_line.as_str() {
                                    "+" => self.result += num,
                                    "-" => self.result -= num,
                                    "*" => self.result *= num,
                                    "/" => {
                                        if num != 0.0 {
                                            self.result /= num
                                        } else {
                                            self.text_result = "ERROR".to_string();
                                        }
                                    }
                                    _ => self.result = num,
                                };
                                self.back_line = "".to_string();
                            } else {
                                self.back_line = line.to_string();
                            };
                        }
                        if self.text_result != "ERROR".to_string() {
                            self.text_result = self.result.to_string();
                        }
                    }
                    if col[3].button("+").clicked() {
                        self.text_result += "\n+\n";
                    }
                });
            });
    }
}
