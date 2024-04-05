use eframe::egui;

use super::View;

const BUTTONS: &str = "7 8 9 +|4 5 6 -|1 2 3 *|0 . = /";

#[derive(Default, Debug)]
pub struct Calculator {
    result_text: String,
}

impl super::Tool for Calculator {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Calculator")
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Calculator {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let text = ui.add(
            egui::TextEdit::multiline(&mut self.result_text)
                .horizontal_align(egui::Align::RIGHT)
                .desired_rows(3)
                .desired_width(f32::INFINITY),
        );

        if text.double_clicked() {
            self.result_text = String::new();
        }

        for row in BUTTONS.split("|") {
            let cols: Vec<&str> = row.split(" ").collect();
            ui.columns(cols.len(), |col| {
                for (index, item) in cols.iter().enumerate() {
                    if col[index].button(item.to_string()).clicked() {
                        match item {
                            &"+" | &"-" | &"*" | &"/" => {
                                self.result_text += format!("\n{}\n", item).as_str()
                            }
                            &"=" => self.result_text = calculate(&self.result_text),
                            _ => self.result_text += item,
                        }
                    }
                }
            });
        }
    }
}

fn calculate(result_text: &String) -> String {
    let mut result = 0.0;
    let mut line_back = String::new();

    for line in result_text.split("\n") {
        if let Ok(num) = line.parse::<f64>() {
            match line_back.as_str() {
                "+" => result += num,
                "-" => result -= num,
                "*" => result *= num,
                "/" => {
                    if num != 0.0 {
                        result /= num
                    } else {
                        return "ERROR".to_string();
                    }
                }
                _ => result = num,
            };
            line_back = "".to_string();
        } else {
            line_back = line.to_string();
        };
    }
    result.to_string()
}
