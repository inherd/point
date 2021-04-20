use druid::{Color, Data, Key, Widget};
use druid_theme_loader::loadable_theme;

pub const BACKGROUND_COLOR: Key<Color> = Key::new("com.inherd.print.theme.bg-color");

include!(concat!(env!("OUT_DIR"), "/theme_path.rs"));

// declares a new struct, MyTheme.
loadable_theme!(pub MyTheme {
    BACKGROUND_COLOR,
});

pub fn wrap_in_theme_loader<T: Data>(widget: impl Widget<T>) -> impl Widget<T> {
    druid_theme_loader::ThemeLoader::new(THEME_FILE_PATH, MyTheme, widget)
}
