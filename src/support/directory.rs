extern crate dirs;

use crate::app_state::{AppState, Workspace};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn save_config(state: &AppState) {
    let mut current_state = state.clone();
    current_state.workspace = Workspace::default();

    let result = serde_json::to_string(&current_state);
    match result {
        Ok(str) => {
            let path = config_path().expect("lost home issue");
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&path)
                .expect("unable to open file");

            let result = file.write_all(str.as_bytes());

            match result {
                Ok(_) => log::info!("save file: {:?}", path),
                Err(e) => log::info!("failed to write data: {}", { e }),
            }
        }
        Err(err) => {
            log::info!("serialize config error: {:?}", err);
        }
    }
}

fn config_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let base = home.join(".print");
    if !&base.exists() {
        let _ = fs::create_dir_all(&base);
    }
    let config_path = base.join("print.json");
    Some(config_path)
}
