use druid::{Color, Data, Key, Widget};
use druid_theme_loader::loadable_theme;

pub const SIDEBAR_BACKGROUND: Key<Color> = Key::new("print.sidebar-background");
pub const SIDEBAR_EDGE_STROKE: Key<Color> = Key::new("print.sidebar-edge-stroke");

pub const GLYPH_LIST_BACKGROUND: Key<Color> = Key::new("print.background");
pub const TOOL_WINDOW_COLOR: Key<Color> = Key::new("print.tool-window-color");

pub const BACKGROUND_COLOR: Key<Color> = Key::new("print.theme.bg-color");
pub const BUTTON_DARK: Key<Color> = Key::new("org.linebender.druid.theme.button_dark");
pub const BUTTON_LIGHT: Key<Color> = Key::new("org.linebender.druid.theme.button_light");

include!(concat!(env!("OUT_DIR"), "/theme_path.rs"));

// declares a new struct, MyTheme.
loadable_theme!(pub MyTheme {
    SIDEBAR_BACKGROUND,
    SIDEBAR_EDGE_STROKE,
    GLYPH_LIST_BACKGROUND,
    BACKGROUND_COLOR,
    TOOL_WINDOW_COLOR,
    BUTTON_LIGHT,
    BUTTON_DARK
});

pub fn wrap_in_theme_loader<T: Data>(widget: impl Widget<T>) -> impl Widget<T> {
    druid_theme_loader::ThemeLoader::new(THEME_FILE_PATH, MyTheme, widget)
}
