use eframe::egui;

mod config;
mod config_types;
mod value;
mod value_types;

use config::QuestionConfig;
use value::QuestionValue;

#[derive(Clone, Debug)]
pub struct Question {
    pub name: String,
    pub config: QuestionConfig,
    pub value: QuestionValue,
}

impl Question {
    pub fn new() -> Self {
        Self {
            name: "Question".into(),
            config: QuestionConfig::ShortAnswer(config_types::ShortAnswerConfig::default()),
            value: QuestionValue::ShortAnswer(value_types::ShortAnswerValue::default()),
        }
    }

    pub fn types_list() -> Vec<(QuestionConfig, QuestionValue)> {
        QuestionConfig::types_list()
            .iter()
            .zip(QuestionValue::types_list())
            .into_iter()
            .map(|(config, value)| (config.clone(), value))
            .collect()
    }

    pub fn reset_value(&mut self) {
        match (&mut self.config, &mut self.value) {
            (QuestionConfig::ShortAnswer(_), QuestionValue::ShortAnswer(value)) => {
                value.text = String::new();
            }
            (QuestionConfig::Paragraph(_), QuestionValue::Paragraph(value)) => {
                value.text = String::new();
            }
            (QuestionConfig::MultipleChoice(_), QuestionValue::MultipleChoice(value)) => {
                value.choice = String::new();
            }
            (QuestionConfig::Checkboxes(config), QuestionValue::Checkboxes(value)) => {
                value.choices = vec![false; config.options.len()];
            }
            (QuestionConfig::Dropdown(_), QuestionValue::Dropdown(value)) => {
                value.choice = String::new();
            }
            (QuestionConfig::LinearScale(config), QuestionValue::LinearScale(value)) => {
                value.value = config.start;
            }
            (
                QuestionConfig::MultipleChoiceGrid(config),
                QuestionValue::MultipleChoiceGrid(value),
            ) => {
                value.choices = vec![String::new(); config.rows.len()];
            }
            (QuestionConfig::CheckboxGrid(config), QuestionValue::CheckboxGrid(value)) => {
                value.choices = vec![vec![false; config.columns.len()]; config.rows.len()];
            }
            (QuestionConfig::Date(_), QuestionValue::Date(value)) => {
                value.year = 0;
                value.month = 1;
                value.day = 1;
            }
            (QuestionConfig::Time(_), QuestionValue::Time(value)) => {
                value.hour = 1;
                value.minute = 0;
                value.period = value_types::DayPeriod::AM;
            }
            _ => panic!("Config type is not the same as value type"),
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
                        ui.menu_button(self.config.name(), |ui| {
                            ui.set_width(120.0);
                            for (config, value) in Self::types_list() {
                                if ui.button(config.name()).clicked() {
                                    self.config = config;
                                    self.value = value;
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
            self.config.edit(ui);
        });
        should_delete
    }

    pub fn preview(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(self.name.clone());
            match (&mut self.config, &mut self.value) {
                (QuestionConfig::ShortAnswer(_), QuestionValue::ShortAnswer(value)) => {
                    ui.add(egui::TextEdit::singleline(&mut value.text).hint_text("Your answer"));
                }
                (QuestionConfig::Paragraph(_), QuestionValue::Paragraph(value)) => {
                    ui.add(egui::TextEdit::multiline(&mut value.text).hint_text("Your answer"));
                }
                (QuestionConfig::MultipleChoice(config), QuestionValue::MultipleChoice(value)) => {
                    for option in config.options.iter() {
                        ui.radio_value(&mut value.choice, option.clone(), option);
                    }
                }
                (QuestionConfig::Checkboxes(config), QuestionValue::Checkboxes(value)) => {
                    for (i, option) in config.options.iter().enumerate() {
                        ui.checkbox(&mut value.choices[i], option);
                    }
                }
                (QuestionConfig::Dropdown(config), QuestionValue::Dropdown(value)) => {
                    let label = if value.choice.is_empty() {
                        "Choose".into()
                    } else {
                        value.choice.clone()
                    };
                    ui.menu_button(label, |ui| {
                        for option in config.options.iter() {
                            if ui
                                .selectable_value(&mut value.choice, option.clone(), option)
                                .clicked()
                            {
                                ui.close_menu();
                            }
                        }
                    });
                }
                (QuestionConfig::LinearScale(config), QuestionValue::LinearScale(value)) => {
                    ui.horizontal(|ui| {
                        for i in config.start..=config.end {
                            ui.radio_value(&mut value.value, i, i.to_string());
                        }
                    });
                }
                (
                    QuestionConfig::MultipleChoiceGrid(config),
                    QuestionValue::MultipleChoiceGrid(value),
                ) => {
                    egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                        ui.label("");
                        for column in config.columns.iter() {
                            ui.label(column);
                        }
                        ui.end_row();
                        for (y, row) in config.rows.iter().enumerate() {
                            ui.label(row);
                            for column in config.columns.iter() {
                                ui.radio_value(&mut value.choices[y], column.clone(), "");
                            }
                            ui.end_row();
                        }
                    });
                }
                (QuestionConfig::CheckboxGrid(config), QuestionValue::CheckboxGrid(value)) => {
                    egui::Grid::new(ui.next_auto_id()).show(ui, |ui| {
                        ui.label("");
                        for column in config.columns.iter() {
                            ui.label(column);
                        }
                        ui.end_row();
                        for (y, row) in config.rows.iter().enumerate() {
                            ui.label(row);
                            for x in 0..config.columns.len() {
                                ui.checkbox(&mut value.choices[y][x], "");
                            }
                            ui.end_row();
                        }
                    });
                }
                (QuestionConfig::Date(_), QuestionValue::Date(value)) => {
                    ui.label("MM  DD  YYYY");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut value.month).clamp_range(1..=12));
                        ui.label("/");
                        ui.add(egui::DragValue::new(&mut value.day).clamp_range(1..=31));
                        ui.label("/");
                        ui.add(egui::DragValue::new(&mut value.year).clamp_range(0..=9999));
                    });
                }
                (QuestionConfig::Time(_), QuestionValue::Time(value)) => {
                    ui.label("Time");
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut value.hour).clamp_range(1..=12));
                        ui.label(":");
                        ui.add(egui::DragValue::new(&mut value.minute).clamp_range(0..=59));
                        ui.menu_button(
                            match value.period {
                                value_types::DayPeriod::AM => "AM",
                                value_types::DayPeriod::PM => "PM",
                            },
                            |ui| {
                                if ui
                                    .selectable_value(
                                        &mut value.period,
                                        value_types::DayPeriod::AM,
                                        "AM",
                                    )
                                    .clicked()
                                {
                                    ui.close_menu();
                                }
                                if ui
                                    .selectable_value(
                                        &mut value.period,
                                        value_types::DayPeriod::PM,
                                        "PM",
                                    )
                                    .clicked()
                                {
                                    ui.close_menu();
                                }
                            },
                        );
                    });
                }
                _ => panic!("Config type is not the same as value type"),
            }
        });
    }
}
