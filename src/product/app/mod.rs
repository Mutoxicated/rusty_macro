use std::slice::IterMut;

use serde::{Deserialize, Serialize};

use crate::{construct, csmacro::{call::CSMacroCall, definition::CSMacroDefinition}};

#[derive(Deserialize, Serialize)]
pub struct App {
    script_path:Option<String>,

    macro_definitions:Vec<CSMacroDefinition>,
    macro_calls:Vec<CSMacroCall>,

    #[serde(skip_serializing)]
    current_path:String,

    #[serde(skip_serializing)]
    macro_def_err: Option<String>

}

construct!(
    App;
    {
        let current = std::env::current_dir().expect("Unable to get current dir.");
    }
    script_path = None,

    macro_definitions = Vec::new(),
    macro_calls = Vec::new(),

    current_path = current.to_string_lossy().to_string(),
    macro_def_err = None
);

impl App {
    pub fn set_spath(&mut self, path:&str) {
        self.script_path = Some(path.to_owned())
    }

    pub fn spath(&self) -> &Option<String> {
        &self.script_path
    }

    pub fn set_current_path(&mut self, str:&str) {
        self.current_path = str.to_owned();
    }

    pub fn current_path(&self) -> &str {
        self.current_path.as_str()
    }

    pub fn contains_macro_def(&self, name:&str) -> bool {
        for mcdef in &self.macro_definitions {
            if mcdef.name() == name {
                return true
            }
        }
        false
    }

    pub fn add_macro_def(&mut self, macro_def:CSMacroDefinition) {
        if self.contains_macro_def(macro_def.name()) {
            self.macro_def_err = Some(String::from("There is already a macro with that name"));
            return;
        }
        
        self.macro_definitions.push(macro_def);
    }

    pub fn remove_macro_def(&mut self, i:usize) {
        self.macro_definitions.remove(i);
    }

    pub fn macro_def_mut(&mut self, i:usize) -> &mut CSMacroDefinition{
        &mut self.macro_definitions[i]
    }

    pub fn macro_def_iter_mut(&mut self) -> IterMut<'_, CSMacroDefinition> {
        self.macro_definitions.iter_mut()   
    }

    pub fn macro_def_len(&self) -> usize {
        self.macro_definitions.len()
    }
}