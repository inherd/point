use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::client::{Client, RpcOperations};
use crate::model::file_tree::FileEntry;
use crate::support::directory;

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct AppState {
    pub title: String,
    pub workspace: Workspace,
    pub params: Params,
    pub entry: FileEntry,

    #[serde(skip_serializing, skip_deserializing)]
    pub core: Arc<Mutex<Client>>,

    #[serde(default)]
    pub current_file: Option<Arc<Path>>,
    #[serde(default)]
    pub current_dir: Option<Arc<Path>>,

    #[serde(default)]
    pub last_dir: Option<Arc<Path>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            workspace: Default::default(),
            params: Default::default(),
            entry: Default::default(),
            core: Arc::new(Mutex::new(Default::default())),
            current_file: None,
            current_dir: None,
            last_dir: None,
        }
    }
}

impl AppState {
    pub fn set_file(&mut self, path: impl Into<Option<PathBuf>>) {
        let path: Option<Arc<Path>> = path.into().map(Into::into);

        let mut file_content: Vec<u8> = Vec::new();
        let mut file = File::open(&path.as_ref().unwrap()).expect("Unable to open file");
        if let Err(err) = file.read_to_end(&mut file_content) {
            log::error!("open file error: {:?}", err);
            return;
        };

        let out = String::from_utf8_lossy(&*file_content);

        self.workspace.input_text = out.to_string();
        self.workspace.current_file = Arc::new(path.clone().unwrap().to_path_buf());

        self.current_file = path;
        self.save_global_config();
    }

    pub fn reload_dir(&mut self) {
        self.entry = FileEntry::from_dir(
            self.workspace.project.clone(),
            &self.current_dir.as_ref().unwrap(),
        );
    }

    pub fn set_dir(&mut self, path: impl Into<Option<PathBuf>>) {
        let path: Option<Arc<Path>> = path.into().map(Into::into);
        if let Some(dir) = path.clone() {
            if let Some(name) = dir.file_name() {
                self.workspace.project = format!("{}", name.to_str().unwrap());
                self.workspace.dir = Arc::new(dir.clone().to_path_buf());
            }

            self.entry = FileEntry::from_dir(self.workspace.project.clone(), &dir);
            log::info!("open dir: {:?}", dir);
        }

        self.last_dir = self.current_dir.clone();
        self.current_dir = path;

        self.save_global_config();
    }

    pub fn text(&mut self) -> String {
        return self.workspace.input_text.clone();
    }

    // todo: add save project config
    pub fn save_global_config(&mut self) {
        let mut current_state = self.clone();

        current_state.workspace = Default::default();
        current_state.entry = Default::default();

        directory::save_config(&current_state);
    }

    pub fn setup_workspace(&mut self) {
        println!("init state: {:?}", self);
        if let Some(path) = self.current_file.clone() {
            &self.set_file(path.to_path_buf());
        }
        if let Some(path) = self.current_dir.clone() {
            &self.set_dir(path.to_path_buf());
        }
    }

    pub fn handle_event(&self, op: RpcOperations) {
        log::info!("Handling msg: {:?}", op);
        let core = self.core.lock().unwrap();
        match op {
            RpcOperations::AvailableThemes(_themes) => {
                core.send_notification("set_theme", &json!({ "theme_name": "InspiredGitHub" }));
            }
            RpcOperations::AvailableLanguages(_langs) => {}
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct Workspace {
    pub project: String,
    pub origin_text: String,
    pub input_text: String,
    pub char_count: usize,

    #[serde(default)]
    pub dir: Arc<PathBuf>,

    #[serde(default)]
    current_file: Arc<PathBuf>,
}

impl Workspace {
    pub fn relative_path(&self) -> String {
        match self.current_file.strip_prefix(&*self.dir) {
            Ok(path) => {
                let mut paths: Vec<String> = vec![];
                for sub in path.iter() {
                    paths.push(sub.to_str().unwrap().to_string())
                }
                if paths.len() == 0 {
                    return self.project.to_string();
                }
                format!("{} > {}", self.project, paths.join(" > "))
            }
            Err(_) => self.project.to_string(),
        }
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            project: "".to_string(),
            origin_text: "".to_string(),
            input_text: "".to_string(),
            char_count: 0,
            dir: Default::default(),
            current_file: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct Params {
    pub debug_layout: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            debug_layout: false,
        }
    }
}
