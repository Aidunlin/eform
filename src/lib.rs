use eframe::egui;
use serde::{Deserialize, Serialize};

mod gform;

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
    forms: Vec<gform::forms::Form>,
    responses: Vec<gform::responses::FormResponse>,
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

    fn top_panel(
        id: impl Into<egui::Id>,
        context: &egui::Context,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        egui::TopBottomPanel::top(id).show(context, |ui| ui.horizontal(add_contents));
    }

    fn central_panel(context: &egui::Context, add_contents: impl FnOnce(&mut egui::Ui)) {
        egui::CentralPanel::default().show(context, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, add_contents);
        });
    }

    fn new_response(&mut self, form: gform::forms::Form) {
        let response_id = self
            .responses
            .iter()
            .filter(|response| response.form_id == form.form_id)
            .count()
            .to_string();

        let response = gform::responses::FormResponse {
            form_id: form.form_id,
            response_id,
            ..gform::responses::FormResponse::default()
        };

        self.view = AppView::Response(self.responses.len());
        self.responses.push(response);
    }

    fn ui_default(&mut self, context: &egui::Context) {
        Self::top_panel("top_panel_default", context, |ui| {
            if ui.button("New form").clicked() {
                let index = self.forms.len();

                let form = gform::forms::Form {
                    form_id: index.to_string(),
                    info: gform::forms::Info {
                        title: "Untitled form".into(),
                        ..gform::forms::Info::default()
                    },
                    ..gform::forms::Form::default()
                };

                self.view = AppView::Form(index);
                self.forms.push(form.clone());
            }
        });

        Self::central_panel(context, |ui| {
            egui::Grid::new("forms_grid").striped(true).show(ui, |ui| {
                for (index, form) in self.forms.clone().iter().enumerate() {
                    ui.label(form.info.title.clone());

                    ui.horizontal(|ui| {
                        if ui.button("Edit").clicked() {
                            self.view = AppView::Form(index);
                        }

                        if ui.button("Fill").clicked() {
                            self.new_response(form.clone());
                        }

                        if ui.button("Remove").clicked() {
                            self.forms.remove(index);
                            self.responses = self
                                .responses
                                .clone()
                                .into_iter()
                                .filter(|response| response.form_id != form.form_id)
                                .collect();
                        }
                    });

                    ui.end_row();
                }
            });
        });
    }

    fn ui_form(&mut self, context: &egui::Context, form_index: usize) {
        Self::top_panel("top_panel_form", context, |ui| {
            if ui.button("Back").clicked() {
                self.view = AppView::Default;
            }

            if ui.button("Fill").clicked() {
                self.new_response(self.forms[form_index].clone());
            }
        });

        Self::central_panel(context, |ui| {
            ui.group(|ui| {
                ui.text_edit_singleline(&mut self.forms[form_index].info.title);
                ui.text_edit_multiline(&mut self.forms[form_index].info.description);
            });
        });
    }

    fn ui_response(&mut self, context: &egui::Context, response_index: usize) {
        let form_index = self.responses[response_index]
            .form_id
            .parse::<usize>()
            .unwrap();
        let form = self.forms[form_index].clone();

        Self::top_panel("top_panel_response", context, |ui| {
            if ui.button("Back").clicked() {
                self.view = AppView::Default;
            }
        });

        Self::central_panel(context, |ui| {
            ui.group(|ui| {
                ui.heading(form.info.title.clone());

                if form.info.description.len() > 0 {
                    ui.label(form.info.description);
                }
            });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        match self.view {
            AppView::Default => self.ui_default(context),
            AppView::Form(form_index) => self.ui_form(context, form_index),
            AppView::Response(response_index) => self.ui_response(context, response_index),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "data", self);
    }
}
