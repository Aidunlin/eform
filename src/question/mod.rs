mod data;
mod types;

pub use data::QuestionData;
use eframe::{egui::{Ui, emath::Vec2, Layout, Direction}, emath::Align};
use types::ShortAnswer;
pub use types::DayPeriod;

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

    pub fn reset_value(&mut self) {
        match &mut self.data {
            QuestionData::ShortAnswer(data) => {
                data.text = String::new();
            },
            QuestionData::Paragraph(data) => {
                data.text = String::new();
            },
            QuestionData::MultipleChoice(data) => {
                data.choice = String::new();
            },
            QuestionData::Checkboxes(data) => {
                data.choices = vec![false; data.options.len()];
            },
            QuestionData::Dropdown(data) => {
                data.choice = String::new();
            },
            QuestionData::LinearScale(data) => {
                data.value = data.start;
            },
            QuestionData::MultipleChoiceGrid(data) => {
                data.choices = vec![String::new(); data.rows.len()];
            },
            QuestionData::CheckboxGrid(data) => {
                data.choices = vec![vec![false; data.columns.len()]; data.rows.len()];
            },
            QuestionData::Date(data) => {
                data.year = 0;
                data.month = 1;
                data.day = 1;
            },
            QuestionData::Time(data) => {
                data.hour = 1;
                data.minute = 0;
                data.period = DayPeriod::AM;
            },
        }
    }

    pub fn edit(&mut self, ui: &mut Ui) -> bool {
        let mut should_delete = false;

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.allocate_ui_with_layout(Vec2::new(132.0, 20.0), Layout::centered_and_justified(Direction::TopDown).with_cross_align(Align::Min), |ui| {
                    ui.menu_button(self.data.name(), |ui| {
                        ui.set_width(120.0);
                        for question_type in QuestionData::types_list() {
                            if ui
                                .selectable_value(
                                    &mut self.data,
                                    question_type.clone(),
                                    question_type.name(),
                                )
                                .clicked()
                            {
                                ui.close_menu();
                            }
                        }
                    });
                });
                if ui.button("❌").clicked() {
                    should_delete = true;
                }
            });
            self.data.edit(ui);
        });

        should_delete
    }

    pub fn preview(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(self.name.clone());
            self.data.preview(ui);
        });
    }
}
