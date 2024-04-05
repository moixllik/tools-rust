use eframe::egui;
use msedge_tts::*;
use rodio::Source;
use std::io::Cursor;

#[derive(Default)]
pub struct Speecher {
    text: String,
    selected: String,
    pub voices: Vec<String>,
}

impl super::Tool for Speecher {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        use super::View;

        egui::Window::new("Speecher")
            .resizable(false)
            .open(open)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Speecher {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("Voices")
            .selected_text(format!("{}", self.selected))
            .show_ui(ui, |ui| {
                for voice in self.voices.clone() {
                    ui.selectable_value(&mut self.selected, voice.clone(), voice);
                }
            });
        ui.text_edit_multiline(&mut self.text);
        if ui.button("Play").clicked() {
            play(&self.selected, &self.text);
        }
    }
}

pub fn voices_list() -> Vec<String> {
    let mut list: Vec<String> = vec![];
    for voice in voice::get_voices_list().unwrap() {
        let short_name = voice.short_name.unwrap();
        if short_name.contains("-US-") {
            list.push(short_name)
        }
    }
    list
}

fn play(selected: &String, text: &String) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    for voice in voice::get_voices_list().unwrap() {
        if voice.short_name == Some(selected.to_owned()) {
            let config = tts::SpeechConfig::from(&voice);
            let mut tts = tts::client::connect().unwrap();
            let audio = tts.synthesize(text, &config).unwrap();
            let duration = audio.audio_metadata.iter().fold(0, |d, m| d + m.duration);
            let cursor = Cursor::new(audio.audio_bytes);
            let source = rodio::Decoder::new(cursor).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();

            std::thread::sleep(std::time::Duration::from_micros(duration));
            break;
        }
    }
}
