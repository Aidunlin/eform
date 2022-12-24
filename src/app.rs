use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
enum EditTab {
    Questions,
    Preview,
    Responses,
    Settings,
}

impl Default for EditTab {
    fn default() -> Self {
        Self::Questions
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct EformApp {
    forms: Vec<crate::form::Form>,
    form_index: Option<usize>,
    edit_tab: EditTab,
}

impl EformApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let Some(storage) = cc.storage else {
            return Self::default();
        };
        let Some(data) = storage.get_string("data") else {
            return Self::default();
        };
        let Ok(app) = serde_json::from_str(&data) else {
            return Self::default();
        };
        app
    }

    fn main_menu(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Start a new form");
                if ui.button("Blank").clicked() {
                    self.forms.push(crate::form::Form::new());
                    self.form_index = Some(self.forms.len() - 1);
                }
            });
            ui.group(|ui| {
                ui.heading("Forms");
                let mut delete_form = None;
                egui::Grid::new("forms").striped(true).show(ui, |ui| {
                    for (i, form) in self.forms.iter().enumerate() {
                        ui.label(&form.name);
                        if ui.button("Open").clicked() {
                            self.form_index = Some(i);
                        }
                        if ui.button("❌").clicked() {
                            delete_form = Some(i);
                            ui.close_menu();
                        }
                        ui.end_row();
                    }
                });
                if let Some(i) = delete_form {
                    self.forms.remove(i);
                }
            });
        });
    }

    fn edit_form(&mut self, ctx: &egui::Context, form_index: usize) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    self.edit_tab = EditTab::Questions;
                    self.form_index = None;
                }
                ui.text_edit_singleline(&mut self.forms[form_index].name);
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.edit_tab, EditTab::Questions, "Questions");
                if ui
                    .selectable_value(&mut self.edit_tab, EditTab::Preview, "Preview")
                    .clicked()
                {
                    self.reset_form_preview(form_index);
                }
                ui.selectable_value(&mut self.edit_tab, EditTab::Responses, "Responses");
                ui.selectable_value(&mut self.edit_tab, EditTab::Settings, "Settings");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| match self.edit_tab {
            EditTab::Questions => self.tab_questions(ui, form_index),
            EditTab::Preview => self.tab_preview(ui, form_index),
            EditTab::Responses => self.tab_responses(ui, form_index),
            EditTab::Settings => self.tab_settings(ui, form_index),
        });
    }

    fn tab_questions(&mut self, ui: &mut egui::Ui, form_index: usize) {
        let mut delete_question = None;
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (i, question) in self.forms[form_index].questions.iter_mut().enumerate() {
                    if question.edit(ui) {
                        delete_question = Some(i);
                    }
                }
                if ui.button("Add question").clicked() {
                    self.forms[form_index]
                        .questions
                        .push(crate::question::Question::new());
                }

                ui.add_space(32.0);
            });
        if let Some(i) = delete_question {
            self.forms[form_index].questions.remove(i);
        }
    }

    fn tab_preview(&mut self, ui: &mut egui::Ui, form_index: usize) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for question in self.forms[form_index].questions.iter_mut() {
                    question.preview(ui);
                }
                if ui.button("Submit").clicked() {
                    println!("{:#?}", self.forms[form_index]);
                }
                if ui.button("Clear form").clicked() {
                    self.reset_form_preview(form_index);
                }
            });
    }

    fn reset_form_preview(&mut self, form_index: usize) {
        for question in self.forms[form_index].questions.iter_mut() {
            question.reset_value();
        }
    }

    fn tab_responses(&mut self, ui: &mut egui::Ui, _: usize) {
        ui.heading("UNDER CONSTRUCTION");
    }

    fn tab_settings(&mut self, ui: &mut egui::Ui, _: usize) {
        ui.heading("UNDER CONSTRUCTION");
    }
}

impl eframe::App for EformApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        match self.form_index {
            None => self.main_menu(ctx),
            Some(form_index) => self.edit_form(ctx, form_index),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("data", serde_json::to_string(self).unwrap());
    }
}
