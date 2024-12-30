mod prompt;
mod raw_example;
mod request;

use eframe::egui;
use egui::{ScrollArea, TextEdit};
use clipboard::{ClipboardContext, ClipboardProvider};
use serde_yaml::Value;


struct InputField {
    text: String,
    caption: String,
}

enum PopupMessage {
    Success(String),
    Error(String),
    Warning(String),
}


struct GuiApp {
    input_fields: Vec<InputField>,
    result_text: String,
    selected_tab: usize,
    clipboard: ClipboardContext,
    selected_difficulty: usize,
    selected_prompt_length: usize,
    popup_state: Option<PopupMessage>,
}

impl GuiApp {
    fn yaml_to_markdown(&self, yaml_str: &str) -> Result<String, String> {
        // Parse YAML
        let yaml: Value = serde_yaml::from_str(yaml_str)
            .map_err(|e| format!("Invalid YAML format: {}", e))?;

        let yaml_map = yaml.as_mapping()
            .ok_or("Invalid YAML structure: expected mapping at root")?;

        let mut markdown = String::new();

        // Process each main section
        for (key, value) in yaml_map {
            let section_name = key.as_str()
                .ok_or("Invalid section name")?;

            // Skip processing if it's just "Overall"
            if section_name == "Overall" {
                continue;
            }

            // Add section header
            markdown.push_str(&format!("# {}\n", section_name));

            if let Some(section_map) = value.as_mapping() {
                for (subkey, subvalue) in section_map {
                    let subkey_str = subkey.as_str()
                        .ok_or("Invalid subsection name")?;

                    match subkey_str {
                        "Note" => {
                            if let Some(notes) = subvalue.as_sequence() {
                                for note in notes {
                                    if let Some(note_str) = note.as_str() {
                                        markdown.push_str(&format!("- {}\n", note_str));
                                    }
                                }
                            }
                        }
                        "Rating" => {
                            if let Some(rating) = subvalue.as_str() {
                                markdown.push_str(&format!("- Rating: {}\n", rating));
                            }
                        }
                        _ => {}
                    }
                }
            }
            markdown.push('\n');
        }

        // Add Overall at the end if it exists
        if let Some(overall) = yaml_map.get(&Value::String("Overall".to_string())) {
            if let Some(overall_str) = overall.as_str() {
                markdown.push_str(&format!("# Overall\n Difficulty {}\n", overall_str));
            }
        }

        Ok(markdown)
    }
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            input_fields: vec![
                InputField {
                    text: String::new(),
                    caption: "Enter the prompt".to_string(),
                },
                InputField {
                    text: String::new(),
                    caption: "Enter the previous turn (Optional)".to_string(),
                },
                InputField {
                    text: String::new(),
                    caption: "Enter the yaml file created by a LLM".to_string(),
                },
            ],
            result_text: String::new(),
            selected_tab: 0,
            clipboard: ClipboardProvider::new().unwrap(),
            selected_difficulty: 0,
            selected_prompt_length: 0,
            popup_state: None,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let mut show_popup = true;
        if let Some(popup) = &self.popup_state {
            let (title, message) = match popup {
                PopupMessage::Success(msg) => ("Success", msg),
                PopupMessage::Error(msg) => ("Error", msg),
                PopupMessage::Warning(msg) => ("Warning", msg),
            };

            egui::Window::new(title)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(message);
                    if ui.button("OK").clicked() {
                        show_popup = false;
                    }
                });
        }
        if !show_popup {
            self.popup_state = None;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Quick Assessment Generator");

            // Tabs
            ui.horizontal(|ui| {
                if ui.selectable_label(self.selected_tab == 0, "Input").clicked() {
                    self.selected_tab = 0;
                }
                if ui.selectable_label(self.selected_tab == 1, "Results").clicked() {
                    self.selected_tab = 1;
                }
            });

            ui.separator();

            match self.selected_tab {
                0 => {
                    // Input Tab
                    ScrollArea::vertical().show(ui, |ui| {
                        for field in &mut self.input_fields[0..2] {
                            // Add some spacing between fields
                            ui.add_space(8.0);

                            // Text input
                            ui.add_sized(
                                [ui.available_width() - 100.0, 100.0], // Reduce width to make room for button
                                TextEdit::multiline(&mut field.text)
                                    .hint_text(&field.caption),
                            );

                            // Paste button
                            if ui.button("📋 Paste").clicked() {
                                if let Ok(clipboard_content) = self.clipboard.get_contents() {
                                    field.text = clipboard_content;
                                }
                            }

                            // Caption below the input
                            ui.label(&field.caption);

                            // Add a separator between fields
                            ui.separator();
                        }

                        ui.add_space(16.0);
                        ui.horizontal(|ui| {
                            ui.label("Preferred Difficulty:");
                            egui::ComboBox::from_id_source("difficulty_selector")
                                .selected_text(match self.selected_difficulty {
                                    0 => "None",
                                    1 => "Easy",
                                    2 => "Medium",
                                    3 => "Hard",
                                    _ => "None",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.selected_difficulty, 0, "None");
                                    ui.selectable_value(&mut self.selected_difficulty, 1, "Easy");
                                    ui.selectable_value(&mut self.selected_difficulty, 2, "Medium");
                                    ui.selectable_value(&mut self.selected_difficulty, 3, "Hard");
                                });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Preferred Length:");
                            egui::ComboBox::from_id_source("length_selector")
                                .selected_text(match self.selected_prompt_length {
                                    0 => "Short",
                                    1 => "Normal",
                                    2 => "Long",
                                    _ => "Normal",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.selected_prompt_length, 0, "Short");
                                    ui.selectable_value(&mut self.selected_prompt_length, 1, "Normal");
                                    ui.selectable_value(&mut self.selected_prompt_length, 2, "Long");
                                });
                        });

                        if ui.button("Copy Full Prompt").clicked() {

                            let req_content = request::gen_request_content(
                                self.input_fields[0].text.clone(), self.input_fields[1].text.clone(),
                                self.selected_difficulty, self.selected_prompt_length,
                                false,
                            );
                            match req_content {
                                Ok(content) => {
                                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();


                                    ctx.set_contents(content.to_owned()).unwrap();

                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        }

                        if ui.button("Copy Shorten Prompt").clicked() {

                            let req_content = request::gen_request_content(
                                self.input_fields[0].text.clone(), self.input_fields[1].text.clone(),
                                self.selected_difficulty, self.selected_prompt_length,
                                true,
                            );
                            match req_content {
                                Ok(content) => {
                                    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();


                                    ctx.set_contents(content.to_owned()).unwrap();

                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        }


                        ui.add_space(16.0);
                        let field = &mut self.input_fields[2];
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [ui.available_width() - 100.0, 100.0],
                                TextEdit::multiline(&mut field.text)
                                    .hint_text(&field.caption),
                            );
                            if ui.button("📋 Paste").clicked() {
                                if let Ok(clipboard_content) = self.clipboard.get_contents() {
                                    field.text = clipboard_content;
                                }
                            }
                        });
                        ui.label(&field.caption);
                        ui.separator();

                        // YAML conversion buttons
                        ui.horizontal(|ui| {
                            if ui.button("Paste YAML").clicked() {
                                if let Ok(clipboard_content) = self.clipboard.get_contents() {
                                    self.input_fields[2].text = clipboard_content;
                                }
                            }
                            if ui.button("Clear").clicked() {
                                self.input_fields[2].text.clear();
                            }
                            if ui.button("Convert YAML to Markdown").clicked() {
                                let yaml_text = if self.input_fields[2].text.is_empty() {
                                    // If empty, try to get from clipboard
                                    if let Ok(clipboard_content) = self.clipboard.get_contents() {
                                        self.input_fields[2].text = clipboard_content;
                                        self.input_fields[2].text.clone()
                                    } else {
                                        String::new()
                                    }
                                } else {
                                    self.input_fields[2].text.clone()
                                };

                                if !yaml_text.is_empty() {
                                    match self.yaml_to_markdown(&yaml_text) {
                                        Ok(markdown) => {
                                            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                                            ctx.set_contents(markdown.clone()).unwrap();
                                            self.result_text = markdown;
                                            self.selected_tab = 1;
                                        }
                                        Err(e) => {
                                            self.popup_state = Some(PopupMessage::Error(
                                                format!("Failed to convert YAML: {}", e)
                                            ));
                                        }
                                    }
                                } else {
                                    self.popup_state = Some(PopupMessage::Warning(
                                        "No YAML content to convert".to_string()
                                    ));
                                }
                            }
                        });
                    });
                }
                1 => {
                    // Results Tab
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.add_sized(
                            [ui.available_width(), 200.0],
                            TextEdit::multiline(&mut self.result_text)
                                .interactive(false),
                        );

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            if ui.button("📋 Copy to Clipboard").clicked() {
                                if !self.result_text.is_empty() {
                                    self.clipboard.set_contents(self.result_text.clone()).unwrap();
                                }
                            }

                            if ui.button("🔄 Reset All").clicked() {
                                // Clear all input fields
                                for field in &mut self.input_fields {
                                    field.text.clear();
                                }
                                // Reset result text
                                self.result_text.clear();
                                // Reset difficulty selection
                                self.selected_difficulty = 0;

                                self.selected_prompt_length = 0;

                                self.selected_tab = 0;
                            }
                        });
                    });
                }
                _ => unreachable!(),
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Native Rust GUI",
        options,
        Box::new(|_cc| Box::new(GuiApp::default())),
    )
}