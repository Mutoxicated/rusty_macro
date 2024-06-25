use eframe::App;
use egui::Color32;

use crate::{construct, csmacro::definition::CSMacroDefinition, csstruct, helpers};

use super::app;

csstruct!(
    Dresser;
    app:app::App,
    name_buf:String,
    parameters_buf:Vec<String>,
    code_buf:String
);

construct!(
    Dresser;
    app = app::App::new(),
    name_buf = String::new(),
    parameters_buf = Vec::new(),
    code_buf = String::from("Lorem Ipsum n shit")
);

impl Dresser {
    pub fn clear_code_buf(&mut self) {
        self.code_buf = String::from("Lorem Ipsum n shit");
    }
}

impl App for Dresser {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            // SETTINGS
            let mut valid_macro = true;

            ui.label("Choose a path to the csharp file");
            let but = ui.button("Open");
            if but.clicked() {

                let selected = tinyfiledialogs::open_file_dialog("Rusty Locounter", self.app.current_path(), None);
                if let Some(str) = selected {
                    self.app.set_spath(str.as_str());
                    self.app.set_current_path(str.as_str());
                }
            }

            if let Some(x) = self.app.spath() {
                ui.label(x);
            }
            
            ui.menu_button("Add macro definition", |mui| {
                mui.text_edit_singleline(&mut self.name_buf);
                if self.name_buf.is_empty() {
                    mui.colored_label(Color32::RED, "Macro name is empty!");
                    valid_macro = false;
                }
                mui.label("Parameters");
                let param = mui.button("New");
                if param.clicked() {
                    self.parameters_buf.push("".to_owned())
                }
                let buflen = self.parameters_buf.len();
                for i in 0..buflen {
                    mui.text_edit_singleline(&mut self.parameters_buf[i]);
                }
                
                if helpers::identical_elems(&self.parameters_buf) {
                    mui.colored_label(Color32::RED, "Some parameters have the same name!");
                    valid_macro = false
                }

                mui.label("Code");
                mui.code_editor(&mut self.code_buf);

                let but = mui.button("Add");
                if but.clicked() && valid_macro {
                    self.app.add_macro_def(
                        CSMacroDefinition::new(&self.name_buf)
                            .set_params(self.parameters_buf.clone())
                            .set_code(self.code_buf.clone())
                    );
                    self.clear_code_buf();
                    self.parameters_buf.clear();
                    self.name_buf = String::new();
                }
            });

            if self.app.macro_def_len() != 0 {
                ui.colored_label(Color32::GREEN, "--Macro definitions");
                for i in 0..self.app.macro_def_len() {
                    let macdef = self.app.macro_def_mut(i);
                    ui.label("-".to_owned()+macdef.name());
                    ui.label("Params:");
                    let iter = macdef.params_iter_mut();
                    for param in iter {
                        ui.text_edit_singleline(param);
                    }

                    ui.label("Code:");
                    ui.code_editor(macdef.code_string_mut());
                    let remove = ui.button("Remove");
                    if remove.clicked() {
                        self.app.remove_macro_def(i);
                        break;
                    }
                }
            }
        });
    }
}