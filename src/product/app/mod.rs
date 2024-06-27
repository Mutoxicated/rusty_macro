pub mod state;
pub mod executer;

use std::{fs::File, io::{Read, Write}, path::PathBuf};
use serde::{Deserialize, Serialize};
use state::State;

use crate::construct;

#[derive(Deserialize, Serialize)]
pub struct App {
    pub states:Vec<State>,
    pub current:usize,

    #[serde(skip)]
    pub current_path:String
}

construct!(
    App;
    {
        let res = App::read_saved_data();

        if let Some(x) = res {
            return x
        }

        let current = std::env::current_dir().expect("Unable to get current dir.");
    }
    current_path = current.to_string_lossy().to_string(),
    states = vec![
        State::new()
    ],
    current = 0
);

impl App {
    pub fn create_data_path() -> Option<PathBuf> {
        if let Some(dir) = directories::BaseDirs::new() {
            let config_path = dir.config_dir();
            return Some(config_path.join("Rusty_MacroData"))
        }
        println!("Failed to get the base directory");
        None
    }

    pub fn save_data(&self) {
        let path = App::create_data_path();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        let result = std::fs::read_dir(&path);
        if result.is_err() {
            let result = std::fs::create_dir(&path);
            if let Err(e) = result {
                println!("{}", e);
                return;
            }
        }

        let abs_path = path.canonicalize().expect("Failed to canonicalize path");
        assert!(abs_path.exists());

        let result = File::options()
            .read(false)
            .write(true)
            .truncate(true)
            .create(true)
            .open(abs_path.join("app.json"));
        if let Err(e) = result {
            println!("Failed to create file: {}", e);
            return;
        }

        let mut file = result.unwrap();

        let string = serde_json::to_string_pretty(self).unwrap();

        let res = file.write_all(string.as_bytes());
        if let Err(e) = res {
            println!("Failed to write file: {}", e);
        }
    }

    pub fn read_saved_data() -> Option<Self> {
        let path = App::create_data_path();
        path.as_ref()?;

        let path = path.unwrap();
        let result = std::fs::read_dir(&path);
        if result.is_err() {
            return None
        }

        let abs_path = path.canonicalize().expect("Failed to canonicalize path");
        let result = File::options()
            .read(true)
            .write(false)
            .open(abs_path.join("app.json"));
        if result.is_err() {
            return None
        }
        let mut file = result.unwrap();

        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf);

        let res = serde_json::from_str(buf.as_str());
        if res.is_err() {
            return None
        }

        Some(res.unwrap())
    }

    pub fn set_current_path(string:&mut String, str:&str) {
        *string = str.to_owned();
    }
}