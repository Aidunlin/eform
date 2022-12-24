use eframe::egui;

mod data;
mod types;

pub use data::QuestionData;

#[derive(Clone, Debug)]
pub struct Question {
    pub name: String,
    pub data: QuestionData,
}

impl Question {
    pub fn new() -> Self {
        Self {
            name: "Question".into(),
            data: QuestionData::ShortAnswer(types::ShortAnswer::default()),
        }
    }

    pub fn edit(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_delete = false;
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.allocate_ui_with_layout(
                    eframe::epaint::vec2(132.0, 20.0),
                    egui::Layout::top_down_justified(eframe::emath::Align::Min),
                    |ui| {
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
                    },
                );
                if ui.button("❌").clicked() {
                    should_delete = true;
                }
            });
            self.data.edit(ui);
        });
        should_delete
    }

    pub fn preview(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(self.name.clone());
            self.data.preview(ui);
        });
    }
}
