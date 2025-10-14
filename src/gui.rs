use crate::config::Config;
use crate::dinkdonk;
use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::egui;
use tokio::runtime::Runtime;

struct App {
    config: Config,
    has_key: bool,
    key_input: String,
    rating_title: String,
    randomize: bool,
    broadcast: bool,
    rating_link: String,
    rt: Runtime,
    clip_ctx: ClipboardContext,
    status: String,
}
impl Default for App {
    fn default() -> Self {
        let mut config = Config::default();
        let _ = config.get_or_build_path();

        let has_key = match config.read_file() {
            Ok(_) => !config.apikey.is_empty(),
            Err(_) => false,
        };
        let rt = Runtime::new().expect("Error ");
        Self {
            config,
            has_key,
            key_input: String::new(),
            rating_title: String::new(),
            randomize: false,
            broadcast: false,
            rating_link: String::new(),
            rt,
            clip_ctx: ClipboardContext::new().unwrap(),
            status: String::new(),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rating Poll");

            if self.has_key {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.randomize, "Randomize?");
                    ui.checkbox(&mut self.broadcast, "Broadcast?");
                });
                ui.label("Enter Rating Poll Title");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.rating_title);
                    if ui.button("Post Poll").clicked() {
                        match self.rt.block_on(dinkdonk::Dinkdonk::create_rating_poll(
                            self.rating_title.clone(),
                            self.config.apikey.clone(),
                            self.randomize,
                            self.broadcast,
                        )) {
                            Ok(link) => self.rating_link = link,
                            Err(_) => println!("Error"),
                        }
                    }
                    ui.label(format!("Poll Link: {}", self.rating_link));
                    if ui.button("Copy").clicked() {
                        self.clip_ctx
                            .set_contents(self.rating_link.clone())
                            .expect("Error Copying to Clipboard");
                    }
                });
                return;
            }

            ui.label("Enter your dinkdonk.mov API Key");
            ui.add(
                egui::TextEdit::singleline(&mut self.key_input)
                    .password(true)
                    .hint_text("dinkdonk.mov/api"),
            );

            if ui.button("Save Key").clicked() {
                match self.config.check_key(self.key_input.clone()) {
                    Ok(_) => {
                        if let Ok(_) = self.config.save_file() {
                            self.has_key = true;
                        }
                    }
                    Err(e) => {
                        self.status = format!("Error: {}", e);
                    }
                }
            }
            ui.label(&self.status);
        });
    }
}
pub fn start() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 430.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Poll Manager",
        options,
        Box::new(|_| Ok(Box::<App>::default())),
    )
    .unwrap();
}
