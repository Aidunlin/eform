use eframe::egui;
use serde::{Deserialize, Serialize};

use super::config_types;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QuestionConfig {
    ShortAnswer(config_types::ShortAnswerConfig),
    Paragraph(config_types::ParagraphConfig),
    MultipleChoice(config_types::MultipleChoiceConfig),
    Checkboxes(config_types::CheckboxesConfig),
    Dropdown(config_types::DropdownConfig),
    LinearScale(config_types::LinearScaleConfig),
    MultipleChoiceGrid(config_types::MultipleChoiceGridConfig),
    CheckboxGrid(config_types::CheckboxGridConfig),
    Date(config_types::DateConfig),
    Time(config_types::TimeConfig),
}

impl QuestionConfig {
    pub fn name(&self) -> String {
        match self {
            Self::ShortAnswer(_) => "Short answer".into(),
            Self::Paragraph(_) => "Paragraph".into(),
            Self::MultipleChoice(_) => "Multiple choice".into(),
            Self::Checkboxes(_) => "Checkboxes".into(),
            Self::Dropdown(_) => "Dropdown".into(),
            Self::LinearScale(_) => "Linear scale".into(),
            Self::MultipleChoiceGrid(_) => "Multiple choice grid".into(),
            Self::CheckboxGrid(_) => "Checkbox grid".into(),
            Self::Date(_) => "Date".into(),
            Self::Time(_) => "Time".into(),
        }
    }

    pub fn types_list() -> Vec<Self> {
        vec![
            Self::ShortAnswer(config_types::ShortAnswerConfig::default()),
            Self::Paragraph(config_types::ParagraphConfig::default()),
            Self::MultipleChoice(config_types::MultipleChoiceConfig::default()),
            Self::Checkboxes(config_types::CheckboxesConfig::default()),
            Self::Dropdown(config_types::DropdownConfig::default()),
            Self::LinearScale(config_types::LinearScaleConfig::default()),
            Self::MultipleChoiceGrid(config_types::MultipleChoiceGridConfig::default()),
            Self::CheckboxGrid(config_types::CheckboxGridConfig::default()),
            Self::Date(config_types::DateConfig::default()),
            Self::Time(config_types::TimeConfig::default()),
        ]
    }

    pub fn edit(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::ShortAnswer(_) => {}
            Self::Paragraph(_) => {}
            Self::MultipleChoice(data) => edit_options(ui, &mut data.options, "Option"),
            Self::Checkboxes(data) => edit_options(ui, &mut data.options, "Option"),
            Self::Dropdown(data) => edit_options(ui, &mut data.options, "Option"),
            Self::LinearScale(data) => {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut data.start).clamp_range(0..=1));
                    ui.label("to");
                    ui.add(egui::DragValue::new(&mut data.end).clamp_range(2..=10));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}", data.start));
                    ui.add(
                        egui::TextEdit::singleline(&mut data.start_label)
                            .hint_text("Label (optional)")
                            .desired_width(100.0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}", data.end));
                    ui.add(
                        egui::TextEdit::singleline(&mut data.end_label)
                            .hint_text("Label (optional)")
                            .desired_width(100.0),
                    );
                });
            }
            Self::MultipleChoiceGrid(data) => edit_grid(ui, &mut data.rows, &mut data.columns),
            Self::CheckboxGrid(data) => edit_grid(ui, &mut data.rows, &mut data.columns),
            Self::Date(_) => {}
            Self::Time(_) => {}
        }
    }
}

impl PartialEq for QuestionConfig {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::ShortAnswer(_), Self::ShortAnswer(_)) => true,
            (Self::Paragraph(_), Self::Paragraph(_)) => true,
            (Self::MultipleChoice(_), Self::MultipleChoice(_)) => true,
            (Self::Checkboxes(_), Self::Checkboxes(_)) => true,
            (Self::Dropdown(_), Self::Dropdown(_)) => true,
            (Self::LinearScale(_), Self::LinearScale(_)) => true,
            (Self::MultipleChoiceGrid(_), Self::MultipleChoiceGrid(_)) => true,
            (Self::CheckboxGrid(_), Self::CheckboxGrid(_)) => true,
            (Self::Date(_), Self::Date(_)) => true,
            (Self::Time(_), Self::Time(_)) => true,
            _ => false,
        }
    }
}

fn edit_grid(ui: &mut egui::Ui, rows: &mut Vec<String>, columns: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| edit_options(ui, rows, "Row"));
        ui.vertical(|ui| edit_options(ui, columns, "Column"));
    });
}

fn edit_options(ui: &mut egui::Ui, options: &mut Vec<String>, label: impl Into<String>) {
    let label: String = label.into();
    ui.label(format!("{}s", label));
    let mut delete_option = None;
    for (i, option) in options.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(option).desired_width(100.0));
            if ui.button("❌").clicked() {
                delete_option = Some(i);
            }
        });
    }
    if let Some(i) = delete_option {
        options.remove(i);
    }
    if ui.button(format!("Add {}", label.to_lowercase())).clicked() {
        options.push(format!("{} {}", label, options.len() + 1));
    }
}
