mod data;
mod types;

pub use data::QuestionData;
use eframe::egui::Ui;
use types::ShortAnswer;

#[derive(Clone, Debug)]
pub struct Question {
    pub name: String,
    pub data: QuestionData,
}

impl Question {
    pub fn new() -> Self {
        Self {
            name: "Question".into(),
            data: QuestionData::ShortAnswer(ShortAnswer::default()),
        }
    }

    pub fn edit(&mut self, ui: &mut Ui) -> bool {
        let mut should_delete = false;

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.menu_button(self.data.name(), |ui| {
                    for question_type in QuestionData::types_list() {
                        if ui.button(question_type.name()).clicked() {
                            self.data = question_type.clone();
                            ui.close_menu();
                        }
                    }
                });
            });

            self.data.edit(ui);

            if ui.button("Delete").clicked() {
                should_delete = true;
            }
        });

        should_delete
    }
}
