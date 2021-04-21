use druid::widget::prelude::*;
use druid::widget::{Flex, Label, WidgetExt};
use druid::{AppLauncher, Color, UnitPoint, WindowDesc};

use app_state::{AppState, Workspace};
use print::editor::EditView;
pub use support::line;

use crate::app_state::Params;
use crate::components::icon_button::IconButton;
use crate::delegate::Delegate;
use crate::print::ProjectToolWindow;

pub mod app_state;
pub mod command;
pub mod components;
pub mod delegate;
pub mod menu;
pub mod model;
pub mod print;
pub mod support;
pub mod theme;

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

fn navigation_bar() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _: &Env| match &data.current_dir {
        None => {
            format!("")
        }
        Some(path) => {
            format!("{}", path.to_owned().display())
        }
    });

    Flex::row()
        .with_child(label.with_text_color(Color::BLACK))
        .padding(10.0)
        .expand_width()
        .background(line::hline())
        .align_horizontal(UnitPoint::LEFT)
}

fn status_bar() -> impl Widget<AppState> {
    let label = Label::new("status bar").with_text_color(Color::BLACK);
    Flex::row()
        .with_default_spacer()
        .with_flex_child(label, 1.0)
        .with_default_spacer()
        .with_flex_child(Label::new("time").with_text_color(Color::BLACK), 1.0)
        .lens(AppState::params)
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
}

fn bottom_tool_window() -> impl Widget<AppState> {
    let text = "Run";
    let label = Label::new(text).with_text_color(Color::BLACK);
    let button = IconButton::from_label(label);
    Flex::row()
        .with_default_spacer()
        .with_flex_child(button, 1.0)
        .lens(AppState::params)
        .background(line::hline())
}

fn center() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(ProjectToolWindow::new(), 1.0)
        .with_default_spacer()
        .with_flex_child(EditView::new().center(), 1.0)
        .padding(1.0)
        .background(line::hline())
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(navigation_bar())
        .with_flex_child(center(), 1.0)
        .with_child(bottom_tool_window())
        .with_child(status_bar())
        .background(LIGHTER_GREY)
}

pub fn main() {
    let title = "Print UI";

    let menu = menu::menus();

    let main_window = WindowDesc::new(crate::theme::wrap_in_theme_loader(make_ui()))
        .window_size((720., 600.))
        .with_min_size((620., 300.))
        .menu(menu)
        .title(title);

    let params = Params {
        debug_layout: false,
    };

    let init_state = AppState {
        title: title.to_string(),
        workspace: Workspace::default(),
        params,
        entry: Default::default(),
        current_file: None,
        current_dir: None,
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate::default())
        .log_to_console()
        .launch(init_state)
        .expect("Failed to launch application");
}
