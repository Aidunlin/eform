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

#[derive(Default, PartialEq, Serialize, Deserialize)]
enum FormTab {
    #[default]
    Questions,
    Responses,
    Settings,
}

#[derive(Default, Serialize, Deserialize)]
pub struct App {
    view: AppView,
    form_tab: FormTab,
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
        let options = eframe::NativeOptions::default();
        let app: eframe::AppCreator = Box::new(|context| Box::new(Self::new(context)));
        eframe::run_native("eform", options, app).unwrap();
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

    fn remove_form(&mut self, index: usize) {
        self.responses = self
            .responses
            .clone()
            .into_iter()
            .filter(|response| response.form_id != self.forms[index].form_id)
            .collect();

        self.forms.remove(index);
    }

    fn new_response(&mut self, form: gform::forms::Form) -> usize {
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

        self.responses.push(response);

        self.responses.len() - 1
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
                self.forms.push(form);
            }
        });

        Self::central_panel(context, |ui| {
            egui::Grid::new("forms_grid").striped(true).show(ui, |ui| {
                for index in 0..self.forms.len() {
                    ui.label(&self.forms[index].info.title);

                    ui.horizontal(|ui| {
                        if ui.button("Edit").clicked() {
                            self.view = AppView::Form(index);
                        }

                        if ui.button("Fill").clicked() {
                            let response_index = self.new_response(self.forms[index].clone());
                            self.view = AppView::Response(response_index);
                        }

                        if ui.button("Remove").clicked() {
                            self.remove_form(index);
                        }
                    });

                    ui.end_row();
                }
            });
        });
    }

    fn ui_form(&mut self, context: &egui::Context, form_index: usize) {
        let mut this_form_deleted = false;

        Self::top_panel("top_panel_form", context, |ui| {
            if ui.button("Back").clicked() {
                self.view = AppView::Default;
            }

            ui.separator();

            ui.selectable_value(&mut self.form_tab, FormTab::Questions, "Questions");
            ui.selectable_value(&mut self.form_tab, FormTab::Responses, "Responses");
            ui.selectable_value(&mut self.form_tab, FormTab::Settings, "Settings");

            ui.separator();

            if ui.button("Preview").clicked() {
                let response_index = self.new_response(self.forms[form_index].clone());
                self.view = AppView::Response(response_index);
            }

            if ui.button("Duplicate").clicked() {
                let mut new_form = self.forms[form_index].clone();
                new_form.form_id = self.forms.len().to_string();

                if new_form.info.title.len() > 0 {
                    new_form.info.title += " Copy";
                }

                if new_form.info.document_title.len() > 0 {
                    new_form.info.document_title += " Copy";
                }

                self.forms.push(new_form);
                self.view = AppView::Form(self.forms.len() - 1);
            }

            if ui.button("Remove").clicked() {
                self.remove_form(form_index);
                this_form_deleted = true;
            }
        });

        if this_form_deleted {
            self.view = AppView::Default;
            return;
        }

        match self.form_tab {
            FormTab::Questions => {
                egui::SidePanel::right("right_panel_form_questions")
                    .resizable(false)
                    .show(context, |ui| {
                        if ui.button("Add question").clicked() {}
                        if ui.button("Import questions").clicked() {}
                        if ui.button("Add title and description").clicked() {}
                        if ui.button("Add image").clicked() {}
                        if ui.button("Add video").clicked() {}
                        if ui.button("Add section").clicked() {}
                    });

                Self::central_panel(context, |ui| {
                    ui.group(|ui| {
                        ui.text_edit_singleline(&mut self.forms[form_index].info.title);
                        ui.text_edit_multiline(&mut self.forms[form_index].info.description);
                    });
                });
            }
            FormTab::Responses => {}
            FormTab::Settings => {}
        }
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
