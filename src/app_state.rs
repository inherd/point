use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::model::file_tree::FileEntry;

use druid::{Data, Lens};
use std::fs::DirEntry;
use std::{fs, io};

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
    if !entry.path().is_dir() {
        return false;
    }

    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn path_to_tree(dir: &Arc<Path>) -> FileEntry {
    let mut root = FileEntry::new("root".to_string());

    let _result = visit_dirs(dir, 0, &mut root);

    root
}

fn visit_dirs(dir: &Path, depth: usize, node: &mut FileEntry) -> io::Result<()> {
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
                let entry = &mut FileEntry::new(format!("{}", path.display()));
                visit_dirs(&path, depth, entry)?
            } else {
                let file_name = format!("{}", path.file_name().unwrap().to_str().unwrap());
                node.children.push(FileEntry::new(file_name));
            }
        }
    }
    Ok(())
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
