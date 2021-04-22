use druid::{Color, Data, Key, Widget};
use druid_theme_loader::loadable_theme;

pub const SIDEBAR_BACKGROUND: Key<Color> = Key::new("print.sidebar-background");
pub const SIDEBAR_EDGE_STROKE: Key<Color> = Key::new("print.sidebar-edge-stroke");

pub const TOOL_WINDOW_COLOR: Key<Color> = Key::new("print.tool-window-color");

pub const FOREGROUND_LIGHT: Key<Color> = Key::new("print.theme.foreground_light");
pub const FOREGROUND_DARK: Key<Color> = Key::new("print.theme.foreground_dark");

pub const BACKGROUND_COLOR: Key<Color> = Key::new("print.theme.bg-color");
pub const BUTTON_DARK: Key<Color> = Key::new("print.theme.button-dark");
pub const BUTTON_LIGHT: Key<Color> = Key::new("print.theme.button-light");

pub const BASIC_TEXT_COLOR: Key<Color> = Key::new("print.theme.text-dark");

pub const BORDERED_WIDGET_HEIGHT: Key<f64> = Key::new("print.theme.button-light-height");
pub const BUTTON_BORDER_WIDTH: Key<f64> = Key::new("print.theme.button-border-width");
pub const BASIC_TEXT_SIZE: Key<f64> = Key::new("print.theme.basic-font-size");

include!(concat!(env!("OUT_DIR"), "/theme_path.rs"));

// declares a new struct, MyTheme.
loadable_theme!(pub MyTheme {
    SIDEBAR_BACKGROUND,
    SIDEBAR_EDGE_STROKE,
    BACKGROUND_COLOR,
    TOOL_WINDOW_COLOR,
    BUTTON_LIGHT,
    BUTTON_DARK,
    FOREGROUND_DARK,
    FOREGROUND_LIGHT,
    BASIC_TEXT_COLOR,
    BORDERED_WIDGET_HEIGHT,
    BUTTON_BORDER_WIDTH,
    BASIC_TEXT_SIZE,
});

pub fn wrap_in_theme_loader<T: Data>(widget: impl Widget<T>) -> impl Widget<T> {
    druid_theme_loader::ThemeLoader::new(THEME_FILE_PATH, MyTheme, widget)
}
