use std::fmt;

use crate::components::tree::TreeNode;
use druid::{Data, Lens};

#[derive(Clone, Lens)]
pub struct FileEntry {
    pub name: String,
    pub icon: String,
    pub is_dir: bool,
    pub children: Vec<FileEntry>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: "".to_string(),
            icon: "".to_string(),
            is_dir: false,
            children: vec![],
        }
    }
}

impl FileEntry {
    pub fn new(name: String) -> Self {
        FileEntry {
            name: name,
            icon: "".to_string(),
            is_dir: false,
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
