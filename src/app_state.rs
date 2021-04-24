use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::model::file_tree::FileEntry;

use druid::{Data, Lens};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs::{DirEntry, File};
use std::{fmt, fs, io};

use crate::support::directory;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::io::Read;

#[derive(Serialize, Deserialize, Clone, Data, Lens, Debug)]
pub struct AppState {
    pub title: String,
    pub workspace: Workspace,
    pub params: Params,
    pub entry: FileEntry,

    #[serde(default, skip_serializing, skip_deserializing)]
    pub watcher: FileWatcher,

    #[serde(default)]
    pub current_file: Option<Arc<Path>>,
    #[serde(default)]
    pub current_dir: Option<Arc<Path>>,

    #[serde(default)]
    pub last_dir: Option<Arc<Path>>,
}

#[derive(Lens)]
pub struct FileWatcher {
    pub instance: Arc<RecommendedWatcher>,
}

impl fmt::Debug for FileWatcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("")
    }
}

impl Clone for FileWatcher {
    fn clone(&self) -> Self {
        Self {
            instance: Arc::new(RecommendedWatcher::new_immediate(|_| {}).unwrap()),
        }
    }
}

impl Data for FileWatcher {
    fn same(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self {
            instance: Arc::new(RecommendedWatcher::new_immediate(|_| {}).unwrap()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let watcher = AppState::create_watcher();

        Self {
            title: "".to_string(),
            workspace: Default::default(),
            params: Default::default(),
            entry: Default::default(),
            watcher,
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

    pub fn set_dir(&mut self, path: impl Into<Option<PathBuf>>) {
        let path: Option<Arc<Path>> = path.into().map(Into::into);
        if let Some(dir) = path.clone() {
            if let Some(name) = dir.file_name() {
                self.workspace.project = format!("{}", name.to_str().unwrap());
                self.workspace.dir = Arc::new(dir.clone().to_path_buf());
            }

            self.entry = path_to_tree(self.workspace.project.clone(), &dir);
            log::info!("open dir: {:?}", dir);
        }

        self.last_dir = self.current_dir.clone();
        self.current_dir = path;

        let _result = self.watch_dir();
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

    pub fn reload_dir(&mut self) {
        // rebuild tree
    }

    #[allow(unused_must_use)]
    pub fn watch_dir(&mut self) -> Result<()> {
        // todo: make in watcher
        if let None = self.last_dir {
            return Ok(());
        }

        let current = self.current_dir.as_ref().unwrap();
        log::info!("watch dir: {:?}", current.display());

        Arc::get_mut(&mut self.watcher.instance)
            .unwrap()
            .unwatch(self.last_dir.as_ref().unwrap());

        Arc::get_mut(&mut self.watcher.instance)
            .unwrap()
            .watch(current, RecursiveMode::Recursive)
    }

    pub fn reinit_config(&mut self) {
        println!("init state: {:?}", self);
        if let Some(path) = self.current_file.clone() {
            &self.set_file(path.to_path_buf());
        }
        if let Some(path) = self.current_dir.clone() {
            &self.set_dir(path.to_path_buf());
        }
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    if !entry.path().is_dir() {
        return entry
            .file_name()
            .to_str()
            .map(|s| s == ".DS_Store")
            .unwrap_or(false);
    }

    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn path_to_tree(title: String, dir: &Arc<Path>) -> FileEntry {
    let mut root = FileEntry::new(title);

    let _result = visit_dirs(dir, 0, &mut root, dir);

    root
}

fn visit_dirs(dir: &Path, depth: usize, node: &mut FileEntry, base_dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        let entry_set = fs::read_dir(dir)?; // contains DirEntry
        let mut entries = entry_set
            .filter_map(|v| match v {
                Ok(dir) => {
                    if is_hidden(&dir) {
                        return None;
                    }
                    Some(dir)
                }
                Err(_) => None,
            })
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.path().file_name().cmp(&b.path().file_name()));

        for (_index, entry) in entries.iter().enumerate() {
            let path = entry.path();

            if path.is_dir() {
                let depth = depth + 1;
                let relative_path = path.strip_prefix(base_dir).unwrap();
                let entry = &mut FileEntry::new(format!("{}", relative_path.display()));
                entry.is_dir = true;
                visit_dirs(&path, depth, entry, base_dir)?;
                node.children.push(entry.to_owned());
            } else {
                let entry1 = FileEntry::from_path(path);
                node.children.push(entry1);
            }
        }
    }
    Ok(())
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

impl AppState {
    pub fn create_watcher() -> FileWatcher {
        let mut watcher: RecommendedWatcher =
            // To make sure that the config lives as long as the function
            // we need to move the ownership of the config inside the function
            // To learn more about move please read [Using move Closures with Threads](https://doc.rust-lang.org/book/ch16-01-threads.html?highlight=move#using-move-closures-with-threads)
            Watcher::new_immediate(move |result: Result<Event>| {
                let event = result.unwrap();
                println!("event: {:?}", event);
            }).expect("error");

        watcher
            .configure(Config::PreciseEvents(true))
            .expect("init watcher failure");

        let watcher = FileWatcher {
            instance: Arc::new(watcher),
        };
        watcher
    }
}
