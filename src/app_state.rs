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
            self.entry = self.path_to_tree(dir);
        }
        self.current_file = path;
    }

    pub fn set_dir(&mut self, path: impl Into<Option<PathBuf>>) {
        let path = path.into().map(Into::into);
        if let Some(dir) = &path {
            self.entry = self.path_to_tree(dir);
            log::info!("open dir: {:?}", dir);
        }
        self.current_dir = path;
    }

    fn path_to_tree(&mut self, dir: &Arc<Path>) -> FileEntry {
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

        let _buf = dir.to_path_buf();
        let root = FileEntry::new("".to_string());

        let walker = WalkDir::new(dir).into_iter();

        let mut last_root = root;
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.unwrap();
            let file_name = entry.file_name().to_os_string();
            if entry.file_type().is_dir() {
                //
            }

            last_root
                .children
                .push(FileEntry::new(format!("{:?}", file_name)));
        }

        last_root
    }
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
