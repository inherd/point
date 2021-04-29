use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use crate::linecache::LineCache;
use crate::model::file_tree::FileEntry;
use crate::rpc::client::{Client, RpcOperations};
use crate::support::directory;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct AppState {
    pub title: String,
    pub workspace: Workspace,
    pub params: Params,
    pub entry: FileEntry,

    #[serde(skip_serializing, skip_deserializing)]
    pub core: Arc<Mutex<Client>>,
    #[serde(skip_serializing, skip_deserializing)]
    pub view: Arc<Mutex<ViewCore>>,

    #[serde(default)]
    pub current_file: Option<Arc<Path>>,
    #[serde(default)]
    pub current_dir: Option<Arc<Path>>,

    #[serde(default)]
    pub last_dir: Option<Arc<Path>>,

    #[serde(default)]
    pub view_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct ViewState {
    id: usize,
    filename: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Lens, Debug)]
pub struct ViewCore {
    focused: Option<String>,
    views: HashMap<String, ViewState>,
}

impl Data for ViewCore {
    // todo: add others compare for data
    fn same(&self, other: &Self) -> bool {
        self.focused == other.focused && self.views.len() == other.views.len()
    }
}

impl Default for ViewCore {
    fn default() -> Self {
        ViewCore {
            focused: None,
            views: Default::default(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            workspace: Default::default(),
            params: Default::default(),
            entry: Default::default(),
            core: Arc::new(Mutex::new(Default::default())),
            view: Arc::new(Mutex::new(Default::default())),
            current_file: None,
            current_dir: None,
            last_dir: None,
            view_id: 0,
        }
    }
}

impl AppState {
    pub fn open_file(&mut self, path: impl Into<Option<PathBuf>>) {
        let path: Option<Arc<Path>> = path.into().map(Into::into);

        let mut file_content: Vec<u8> = Vec::new();
        let mut file = File::open(&path.as_ref().unwrap()).expect("Unable to open file");
        if let Err(err) = file.read_to_end(&mut file_content) {
            log::error!("open file error: {:?}", err);
            return;
        };

        let out = String::from_utf8_lossy(&*file_content);

        self.workspace.input_text = out.to_string();
        let buf = path.clone().unwrap().to_path_buf();
        self.workspace.current_file = Arc::new(buf.clone());

        let file_path = buf.display().to_string();

        self.req_new_view(file_path);
        // let view = self.view.clone();
        // if let Some(id) = view.lock().unwrap().focused.clone() {
        //     self.core.lock().unwrap().resize(id, 512, 512);
        // }

        self.current_file = path;
        self.save_global_config();
    }

    fn req_new_view(&self, filename: String) {
        let view = self.view.clone();
        #[rustfmt::skip]
        self.core.lock().unwrap().new_view(filename.clone()
           , move |res| {
                if let Ok(val) = res {
                    let id: Option<String> = serde_json::from_value(val).unwrap();
                    if let Some(view_id) = id {
                        let mut state = view.lock().unwrap();

                        state.focused = Some(view_id.clone());
                        state.views.insert(
                            view_id.clone(),
                            ViewState {
                                id: 0,
                                filename: Option::from(filename),
                            },
                        );
                        // self.
                    }
                }
        });
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
            &self.open_file(path.to_path_buf());
        }
        if let Some(path) = self.current_dir.clone() {
            &self.set_dir(path.to_path_buf());
        }
    }
}

// for xipart
impl AppState {
    pub fn handle_event(&mut self, op: &RpcOperations) {
        let mut core = self.core.lock().unwrap();
        let view = self.view.lock().unwrap();
        match op {
            RpcOperations::AvailableThemes(_themes) => {
                core.send_notification("set_theme", &json!({ "theme_name": "InspiredGitHub" }));
            }
            RpcOperations::AvailablePlugins(_plugins) => {}
            RpcOperations::AvailableLanguages(_langs) => {
                core.send_notification(
                    "set_language",
                    &json!({ "view_id": view.focused.as_ref().unwrap(), "language_id": "Markdown" }),
                );
            }
            RpcOperations::PluginStarted(_plugin) => {
                // todo: migration plugin
            }
            RpcOperations::Update(update) => {
                self.workspace.line_cache.update(update.clone());
            }
            RpcOperations::MeasureWidth((id, measure_width)) => {
                // core.width_measured(*id, 512, 1024);
            }
            _ => {}
        }
    }

    pub fn resize(&self, size: druid::Size) {
        self.core.lock().unwrap().resize(
            "view-id-2".to_string(),
            size.width as u32,
            size.height as u32,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct Workspace {
    pub project: String,
    pub origin_text: String,
    pub input_text: String,
    pub char_count: usize,

    #[serde(skip_serializing, skip_deserializing)]
    pub line_cache: LineCache,

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
            line_cache: Default::default(),
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
