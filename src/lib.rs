use eframe::egui;
use serde::{Deserialize, Serialize};

mod gform;

#[derive(PartialEq, Serialize, Deserialize)]
enum FormTab {
    Questions { focused_question: usize },
    Responses,
    Settings,
}

impl Default for FormTab {
    fn default() -> Self {
        Self::Questions {
            focused_question: 0,
        }
    }
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
enum AppView {
    #[default]
    Default,
    Form {
        index: usize,
        form_tab: FormTab,
    },
    Response(usize),
}

impl AppView {
    fn form(index: usize) -> Self {
        Self::Form {
            index,
            form_tab: FormTab::default(),
        }
    }

    fn get_focused_question(&self) -> usize {
        let Self::Form { index, form_tab } = self else {
            return 0;
        };

        let FormTab::Questions { focused_question } = form_tab else {
            return 0;
        };

        *focused_question
    }

    fn set_focused_question(&mut self, focus: usize) {
        let Self::Form { index, form_tab } = self else {
            return;
        };

        let FormTab::Questions { mut focused_question } = form_tab else {
            return;
        };

        focused_question = focus;
    }
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

        let Some(app) = eframe::get_value(storage, eframe::APP_KEY) else {
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

                self.view = AppView::form(index);
                self.forms.push(form);
            }
        });

        Self::central_panel(context, |ui| {
            egui::Grid::new("forms_grid").striped(true).show(ui, |ui| {
                for index in 0..self.forms.len() {
                    ui.label(&self.forms[index].info.document_title);

                    ui.horizontal(|ui| {
                        if ui.button("Edit").clicked() {
                            self.view = AppView::form(index);
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

            egui::TextEdit::singleline(&mut self.forms[form_index].info.document_title).show(ui);

            ui.separator();

            let focus = self.view.get_focused_question();

            ui.selectable_value(
                &mut self.view,
                AppView::Form {
                    index: form_index,
                    form_tab: FormTab::Questions {
                        focused_question: focus,
                    },
                },
                "Questions",
            );
            ui.selectable_value(
                &mut self.view,
                AppView::Form {
                    index: form_index,
                    form_tab: FormTab::Responses,
                },
                "Responses",
            );
            ui.selectable_value(
                &mut self.view,
                AppView::Form {
                    index: form_index,
                    form_tab: FormTab::Settings,
                },
                "Settings",
            );

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
                self.view = AppView::form(self.forms.len() - 1);
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

        if let AppView::Form { index: _, form_tab } = &self.view {
            match form_tab {
                FormTab::Questions { focused_question } => {
                    self.ui_form_questions(context, form_index)
                }
                FormTab::Responses => self.ui_form_responses(context, form_index),
                FormTab::Settings => self.ui_form_settings(context, form_index),
            }
        }
    }

    fn ui_form_sidebar(&mut self, ui: &mut egui::Ui, form_index: usize) {
        ui.group(|ui| {
            ui.set_width(25.0);
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    if ui.button("⊞").on_hover_text("Add question").clicked() {}
                    if ui.button("⮋").on_hover_text("Import questions").clicked() {}
                    if ui
                        .button("🇹")
                        .on_hover_text("Add title and description")
                        .clicked()
                    {}
                    if ui.button("🖼").on_hover_text("Add image").clicked() {}
                    if ui.button("🎞").on_hover_text("Add video").clicked() {}
                    if ui.button("➗").on_hover_text("Add section").clicked() {}
                },
            );
        });
    }

    fn ui_form_questions(&mut self, context: &egui::Context, form_index: usize) {
        Self::central_panel(context, |ui| {
            egui::Grid::new("form_questions_grid")
                .min_col_width(300.0)
                .show(ui, |ui| {
                    if ui
                        .group(|ui| {
                            ui.vertical(|ui| {
                                egui::TextEdit::singleline(&mut self.forms[form_index].info.title)
                                    .hint_text("Form title")
                                    .show(ui);
                                egui::TextEdit::multiline(
                                    &mut self.forms[form_index].info.description,
                                )
                                .hint_text("Form description")
                                .desired_rows(1)
                                .show(ui);
                            });
                        })
                        .response
                        .clicked()
                    {
                        self.view.set_focused_question(0);
                    }

                    if self.view.get_focused_question() == 0 {
                        self.ui_form_sidebar(ui, form_index);
                    }

                    ui.end_row();

                    for index in 0..self.forms[form_index].items.len() {
                        let gform::forms::Item {
                            item_id,
                            title,
                            description,
                            kind,
                        } = &mut self.forms[form_index].items[index];

                        if ui
                            .group(|ui| {
                                ui.vertical(|ui| {
                                    egui::TextEdit::singleline(title)
                                        .hint_text("Question")
                                        .show(ui);
                                    egui::TextEdit::multiline(description)
                                        .hint_text("Description")
                                        .desired_rows(1)
                                        .show(ui);
                                });
                            })
                            .response
                            .clicked()
                        {
                            self.view.set_focused_question(index + 1);
                        }

                        if self.view.get_focused_question() == index + 1 {
                            self.ui_form_sidebar(ui, form_index);
                        }

                        ui.end_row();
                    }
                });
        });
    }

    fn ui_form_responses(&mut self, context: &egui::Context, form_index: usize) {}

    fn ui_form_settings(&mut self, context: &egui::Context, form_index: usize) {}

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
            AppView::Form {
                index: form_index,
                form_tab: _,
            } => self.ui_form(context, form_index),
            AppView::Response(response_index) => self.ui_response(context, response_index),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
