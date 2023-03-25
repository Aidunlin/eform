use eframe::egui;
use serde::{Deserialize, Serialize};

mod gform_models;

#[derive(Default, Serialize, Deserialize)]
pub struct App {
    forms: Vec<gform_models::forms::Form>,
    responses: Vec<gform_models::responses::FormResponse>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let Some(storage) = cc.storage else {
            return Self::default();
        };

        let Some(app) = eframe::get_value(storage, "data") else {
            return Self::default();
        };

        app
    }

    pub fn run() {
        eframe::run_native(
            "eform",
            eframe::NativeOptions::default(),
            Box::new(|cc| Box::new(Self::new(cc))),
        )
        .unwrap();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {}

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "data", self);
    }
}
