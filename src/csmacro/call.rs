use std::{borrow::{Borrow}, slice::IterMut};

use serde::{Deserialize, Serialize};

use super::definition::MacroDefinition;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct MacroCall {
    pub definition:MacroDefinition,
    pub arguments:Vec<String>
}

impl MacroCall {
    pub fn new(definition:MacroDefinition) -> Self {
        let mut arguments:Vec<String> = Vec::new();

        for _ in definition.params_iter() {
            arguments.push("".to_owned());
        }

        Self {
            definition,
            arguments
        }
    }

    pub fn definition(&self) -> &MacroDefinition {
        &self.definition
    }

    pub fn arg_iter_mut(&mut self) -> IterMut<'_, String> {
        self.arguments.iter_mut()
    }

    pub fn expand(&self, tabs:&str) -> String {
        let mut code = self.definition.code().to_owned();
        for i in 0..self.arguments.len() {
            code = code.replace((String::from("$")+self.definition.parameter(i)).as_str(), self.arguments[i].as_str());
        }

        code.insert_str(0, tabs);
        let chars:Vec<char> = code.chars().collect();

        for i in (0..chars.len()).rev() {
            if chars[i] == '\n' {
                code.insert_str(i+1, tabs);
            }
        }
        
        code
    }
}