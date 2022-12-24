use super::form::*;
use super::question::*;
use eframe::egui::Direction;
use eframe::egui::Layout;
use eframe::egui::TopBottomPanel;
use eframe::egui::emath::Vec2;
use eframe::emath::Align;
use eframe::epaint::Color32;
use eframe::{
    egui::{CentralPanel, Context, Grid, ScrollArea, Ui},
    App, Frame,
};

#[derive(PartialEq)]
pub enum EditTab {
    Questions,
    Preview,
    Responses,
    Settings,
}

pub struct EformApp {
    pub forms: Vec<Form>,
    pub form_index: Option<usize>,
    pub edit_tab: EditTab,
}

impl EformApp {
    pub fn new() -> Self {
        let mut form = Form::new();
        form.questions = QuestionData::types_list()
            .iter()
            .map(|data| Question {
                name: "Question".into(),
                data: data.clone(),
            })
            .collect();

        Self {
            forms: vec![form],
            form_index: None,
            edit_tab: EditTab::Questions,
        }
    }

    pub fn main_menu(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.heading("Start a new form");
                if ui.button("Blank").clicked() {
                    self.forms.push(Form::new());
                    self.form_index = Some(self.forms.len() - 1);
                }
            });
    
            ui.group(|ui| {
                ui.heading("Forms");
    
                let mut delete_form = None;
    
                Grid::new("forms").striped(true).show(ui, |ui| {
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

    pub fn edit_form(&mut self, ctx: &Context, form_index: usize) {
        TopBottomPanel::top("top").show(ctx, |ui| {
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
                if ui.selectable_value(&mut self.edit_tab, EditTab::Preview, "Preview").clicked() {
                    self.reset_form_preview(form_index);
                }
                ui.selectable_value(&mut self.edit_tab, EditTab::Responses, "Responses");
                ui.selectable_value(&mut self.edit_tab, EditTab::Settings, "Settings");
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match self.edit_tab {
                EditTab::Questions => self.tab_questions(ui, form_index),
                EditTab::Preview => self.tab_preview(ui, form_index),
                EditTab::Responses => self.tab_responses(ui, form_index),
                EditTab::Settings => self.tab_settings(ui, form_index),
            }
        });
    }

    pub fn tab_questions(&mut self, ui: &mut Ui, form_index: usize) {
        let mut delete_question = None;

        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (i, question) in self.forms[form_index].questions.iter_mut().enumerate() {
                    if question.edit(ui) {
                        delete_question = Some(i);
                    }
                }

                if ui.button("Add question").clicked() {
                    self.forms[form_index].questions.push(Question::new());
                }

                ui.add_space(32.0);
            });

        if let Some(i) = delete_question {
            self.forms[form_index].questions.remove(i);
        }
    }

    pub fn tab_preview(&mut self, ui: &mut Ui, form_index: usize) {
        ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
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

    pub fn tab_responses(&mut self, ui: &mut Ui, form_index: usize) {
        ui.heading("UNDER CONSTRUCTION");
    }

    pub fn tab_settings(&mut self, ui: &mut Ui, form_index: usize) {
        ui.heading("UNDER CONSTRUCTION");
    }
}

impl App for EformApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        match self.form_index {
            None => self.main_menu(ctx),
            Some(form_index) => self.edit_form(ctx, form_index),
        }
    }
}
