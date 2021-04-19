use druid::{
    commands, platform_menus, Data, FileDialogOptions, LocalizedString, MenuDesc, MenuItem, SysMods,
};

pub fn menus<T: Data>() -> MenuDesc<T> {
    let mut menu = MenuDesc::empty();
    #[cfg(target_os = "macos")]
    {
        menu = menu.append(platform_menus::mac::application::default());
    }

    menu.append(file_menu())
}

fn file_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::new(LocalizedString::new("common-menu-file-menu"))
        .append(platform_menus::mac::file::new_file().disabled())
        .append(
            MenuItem::new(
                LocalizedString::new("common-menu-file-open"),
                commands::SHOW_OPEN_PANEL.with(FileDialogOptions::new()),
            )
            .hotkey(SysMods::Cmd, "o"),
        )
        .append_separator()
        .append(platform_menus::mac::file::close())
}
