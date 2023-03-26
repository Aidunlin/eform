use eframe::egui;
use serde::{Deserialize, Serialize};

mod gform_models;

#[derive(Default, Serialize, Deserialize)]
enum AppView {
    #[default]
    Default,
    Form(usize),
    Response(usize),
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

    fn new_response(&mut self, form: gform_models::forms::Form) {
        let response_id = self
            .responses
            .iter()
            .filter(|response| response.form_id == form.form_id)
            .count()
            .to_string();

        let response = gform_models::responses::FormResponse {
            form_id: form.form_id.clone(),
            response_id,
            create_time: String::new(),
            last_submitted_time: String::new(),
            respondent_email: String::new(),
            answers: Vec::new(),
            total_score: None,
        };

        self.view = AppView::Response(self.responses.len());
        self.responses.push(response);
    }

    fn ui_default(&mut self, ui: &mut egui::Ui) {
        if ui.button("New form").clicked() {
            let index = self.forms.len();

            let form = gform_models::forms::Form {
                form_id: index.to_string(),
                info: gform_models::forms::Info {
                    title: "Untitled form".into(),
                    document_title: String::new(),
                    description: String::new(),
                },
                settings: gform_models::forms::FormSettings {
                    quiz_settings: gform_models::forms::QuizSettings { is_quiz: false },
                },
                items: Vec::new(),
                revision_id: String::new(),
                responder_uri: String::new(),
                linked_sheet_id: String::new(),
            };

            self.view = AppView::Form(index);
            self.forms.push(form.clone());
        }

        egui::Grid::new("forms_grid").striped(true).show(ui, |ui| {
            let mut new_response = None;
            let mut remove_form = None;

            for (index, form) in self.forms.iter().enumerate() {
                ui.label(form.info.title.clone());

                ui.horizontal(|ui| {
                    if ui.button("Edit form").clicked() {
                        self.view = AppView::Form(index);
                    }

                    if ui.button("New response").clicked() {
                        new_response = Some(index);
                    }

                    if ui.button("Remove form").clicked() {
                        remove_form = Some(index);
                    }
                });

                ui.end_row();
            }

            if let Some(index) = new_response {
                self.new_response(self.forms[index].clone());
            }

            if let Some(index) = remove_form {
                let form_id = self.forms[index].form_id.clone();
                self.forms.remove(index);

                self.responses = self
                    .responses
                    .clone()
                    .into_iter()
                    .filter(|response| response.form_id != form_id)
                    .collect();
            }
        });
    }

    fn ui_form(&mut self, ui: &mut egui::Ui, form_index: usize) {
        if ui.button("Go back").clicked() {
            self.view = AppView::Default;
        }

        if ui.button("New response").clicked() {
            self.new_response(self.forms[form_index].clone());
        }

        ui.group(|ui| {
            ui.text_edit_singleline(&mut self.forms[form_index].info.title);
            ui.text_edit_multiline(&mut self.forms[form_index].info.description);
        });
    }

    fn ui_response(&mut self, ui: &mut egui::Ui, response_index: usize) {
        let form_index = self.responses[response_index]
            .form_id
            .parse::<usize>()
            .unwrap();
        let form = self.forms[form_index].clone();

        if ui.button("Go back").clicked() {
            self.view = AppView::Default;
        }

        ui.group(|ui| {
            ui.heading(form.info.title.clone());

            if form.info.description.len() > 0 {
                ui.label(form.info.description);
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(context, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| match self.view {
                    AppView::Default => self.ui_default(ui),
                    AppView::Form(form_index) => self.ui_form(ui, form_index),
                    AppView::Response(response_index) => self.ui_response(ui, response_index),
                });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "data", self);
    }
}
