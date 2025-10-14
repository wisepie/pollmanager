use crate::config::Config;
use crate::dinkdonk;
use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::egui;
use std::io::ErrorKind;
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
    clip_ctx: Option<ClipboardContext>,
    status: String,
}
impl Default for App {
    fn default() -> Self {
        let mut config = Config::default();
        let mut status = String::new();

        if let Err(e) = config.get_or_build_path() {
            status = format!("Error creating config directory: {e}");
        }

        let has_key = if status.is_empty() {
            match config.read_file() {
                Ok(_) => !config.apikey.is_empty(),
                Err(e) => {
                    if e.kind() != ErrorKind::NotFound {
                        status = format!("Error reading config: {e}");
                    }
                    false
                }
            }
        } else {
            false
        };

        let clip_ctx = match ClipboardContext::new() {
            Ok(ctx) => Some(ctx),
            Err(e) => {
                if status.is_empty() {
                    status = format!("Clipboard unavailable: {e}");
                }
                None
            }
        };
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        Self {
            config,
            has_key,
            key_input: String::new(),
            rating_title: String::new(),
            randomize: false,
            broadcast: false,
            rating_link: String::new(),
            rt,
            clip_ctx,
            status,
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
                            Ok(link) => {
                                self.rating_link = link;
                                self.status.clear();
                            }
                            Err(e) => {
                                self.status = format!("Failed to post poll: {e}");
                            }
                        }
                    }
                    ui.label(format!("Poll Link: {}", self.rating_link));
                    if ui.button("Copy").clicked() {
                        match self.clip_ctx.as_mut() {
                            Some(ctx) => {
                                if let Err(e) = ctx.set_contents(self.rating_link.clone()) {
                                    self.status =
                                        format!("Clipboard unavailable right now: {e}");
                                } else {
                                    self.status.clear();
                                }
                            }
                            None => {
                                self.status = String::from(
                                    "Clipboard access failed; restart or close other clipboard tools",
                                );
                            }
                        }
                    }
                });
                ui.label(&self.status);
                return;
            }

            ui.label("Enter your dinkdonk.mov API Key");
            ui.add(
                egui::TextEdit::singleline(&mut self.key_input)
                    .password(true)
                    .hint_text("dinkdonk.mov/api"),
            );

            if ui.button("Save Key").clicked() {
                if self.config.path.is_none() {
                    self.status = String::from(
                        "Config directory unavailable; restart the app after fixing permissions",
                    );
                } else {
                    match self.config.check_key(self.key_input.clone()) {
                        Ok(_) => match self.config.save_file() {
                            Ok(_) => {
                                self.has_key = true;
                                self.status.clear();
                            }
                            Err(e) => {
                                self.status = format!("Failed to save config: {e}");
                            }
                        },
                        Err(e) => {
                            self.status = format!("Error: {}", e);
                        }
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
