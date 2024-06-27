use std::{borrow::{Borrow, BorrowMut}, collections::{hash_map::IterMut, HashMap}};

use serde::{Deserialize, Serialize};

use super::definition::MacroDefinition;

#[derive(Deserialize, Serialize)]
pub struct MacroCall {
    definition:MacroDefinition,
    arguments:HashMap<String, String>
}

impl MacroCall {
    pub fn new(definition:MacroDefinition) -> Self {
        let mut arguments:HashMap<String, String> = HashMap::new();

        for param in definition.params_iter() {
            arguments.insert(param.clone(), "".to_owned());
        }

        Self {
            definition,
            arguments
        }
    }

    pub fn definition(&self) -> &MacroDefinition {
        &self.definition
    }

    pub fn arg_iter_mut(&mut self) -> IterMut<'_, String, String> {
        self.arguments.borrow_mut().iter_mut()
    }

    pub fn expand(&self) -> String {
        let mut code = self.definition.code().to_owned();
        for (k, v) in self.arguments.borrow() {
            code = code.replace((String::from("$")+k.as_str()).as_str(), v.as_str());
        }
        code
    }
}