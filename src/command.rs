pub mod print_command {
    use crate::model::file_tree::FileEntry;
    use druid::Selector;

    pub const REBUILD_MENUS: Selector = Selector::new("print.rebuild-menus");
    pub const OPEN: Selector = Selector::new("print.open-project");
    pub const OPEN_FILE: Selector<FileEntry> = Selector::new("print.open-file");
}
