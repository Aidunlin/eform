use eframe::egui;
use serde::{Deserialize, Serialize};

mod gform_models;

#[derive(Default, Serialize, Deserialize)]
enum AppView {
    #[default]
    Default,
    Form(gform_models::forms::Form),
    Response(gform_models::responses::FormResponse),
}

#[derive(Default, Serialize, Deserialize)]
pub struct App {
    view: AppView,
    forms: Vec<gform_models::forms::Form>,
    responses: Vec<gform_models::responses::FormResponse>,
}

impl App {
    fn new(context: &eframe::CreationContext<'_>) -> Self {
        let Some(storage) = context.storage else {
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

    fn ui_default(&mut self, ui: &mut egui::Ui) {}

    fn ui_form(&mut self, ui: &mut egui::Ui, form: gform_models::forms::Form) {}

    fn ui_response(&mut self, ui: &mut egui::Ui, response: gform_models::responses::FormResponse) {}
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match &self.view {
                    AppView::Default => self.ui_default(ui),
                    AppView::Form(form) => self.ui_form(ui, form.clone()),
                    AppView::Response(response) => self.ui_response(ui, response.clone()),
                });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "data", self);
    }
}
