use std::path::{Path, PathBuf};
use std::sync::Arc;

use walkdir::{DirEntry, WalkDir};

use crate::model::file_tree::FileEntry;

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub title: String,
    pub workspace: Workspace,
    pub params: Params,
    pub entry: FileEntry,
    pub current_file: Option<Arc<Path>>,
    pub current_dir: Option<Arc<Path>>,
}

impl AppState {
    pub fn set_file(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        if let Some(dir) = &path {
            self.entry = path_to_tree(dir);
        }
        self.current_file = path;
    }

    pub fn set_dir(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        if let Some(dir) = &path {
            self.entry = path_to_tree(dir);
            log::info!("open dir: {:?}", dir);
        }

        self.current_dir = path;
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
        return false;
    }

    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn path_to_tree(dir: &Arc<Path>) -> FileEntry {
    // todo: change root to project name
    let mut root = FileEntry::new("root".to_string());

    let walker = WalkDir::new(dir).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_os_string();

        let relative_path = entry.path().strip_prefix(&dir).unwrap();
        let last_node = &mut root;
        for path in relative_path.iter() {
            let mut file_entry = FileEntry::new(format!("{}", path.to_str().unwrap()));
            last_node.children.push(file_entry);
        }

        print!("{:?}", file_name);
    }

    root
}

#[derive(Clone, Data, Lens)]
pub struct Workspace {
    pub input_text: String,
}

impl Workspace {}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            input_text: "".to_string(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Params {
    pub debug_layout: bool,
}
