use std::fmt;

use crate::components::tree::TreeNode;
use druid::{Data, Lens};
use std::path::PathBuf;

#[derive(Clone, Lens, Debug)]
pub struct FileEntry {
    pub name: String,
    pub ext: String,
    pub is_dir: bool,
    pub path: String,
    pub children: Vec<FileEntry>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: "".to_string(),
            ext: "".to_string(),
            is_dir: false,
            path: "".to_string(),
            children: vec![],
        }
    }
}

impl FileEntry {
    pub fn from_path(path: PathBuf) -> Self {
        let file_name = path.file_name().unwrap();
        let name = match file_name.to_str() {
            None => "".to_string(),
            Some(na) => na.to_string(),
        };
        let ext = match path.extension() {
            None => "".to_string(),
            Some(ext) => ext.to_str().unwrap().to_string(),
        };

        let path = format!("{}", path.display());

        FileEntry {
            name,
            ext,
            is_dir: false,
            path,
            children: vec![],
        }
    }
    pub fn new(name: String) -> Self {
        FileEntry {
            name: name,
            ext: "".to_string(),
            is_dir: false,
            path: "".to_string(),
            children: vec![],
        }
    }

    pub fn add_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }
}

impl Data for FileEntry {
    fn same(&self, other: &Self) -> bool {
        self.name.same(&other.name)
            && self.children.len() == other.children.len()
            && self
                .children
                .iter()
                .zip(other.children.iter())
                .all(|(a, b)| a.same(b))
    }
}

impl TreeNode for FileEntry {
    fn children_count(&self) -> usize {
        self.children.len()
    }

    fn get_child(&self, index: usize) -> &FileEntry {
        &self.children[index]
    }

    fn get_child_mut(&mut self, index: usize) -> &mut FileEntry {
        &mut self.children[index]
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
