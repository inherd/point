use druid::widget::prelude::*;
use druid::widget::{Flex, Label, WidgetExt};
use druid::{AppLauncher, Color, Data, Lens, UnitPoint, WindowDesc};

use print_ui::editor::EditView;

use crate::delegate::Delegate;
use std::path::PathBuf;
pub use support::line;

pub mod command;
pub mod delegate;
pub mod menu;
pub mod print_ui;
pub mod support;
pub mod theme;

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

#[derive(Clone, Data, Lens)]
struct AppState {
    title: String,
    workspace: Workspace,
    params: Params,
}

#[derive(Clone, Data, Lens)]
struct Workspace {
    pub input_text: String,
}

impl Workspace {
    pub fn set_file(&self, _path: impl Into<Option<PathBuf>>) {}
}

#[derive(Clone, Data, Lens)]
struct Params {
    debug_layout: bool,
}

fn navigation_bar() -> impl Widget<AppState> {
    Flex::row()
        .with_child(Label::new("print/src/main.rs").with_text_color(Color::BLACK))
        .padding(10.0)
        .expand_width()
        .lens(AppState::params)
        .background(line::hline())
        .align_horizontal(UnitPoint::LEFT)
}

fn status_bar() -> impl Widget<AppState> {
    Flex::row()
        .with_default_spacer()
        .with_flex_child(Label::new("status bar").with_text_color(Color::BLACK), 1.0)
        .with_default_spacer()
        .with_flex_child(Label::new("time").with_text_color(Color::BLACK), 1.0)
        .lens(AppState::params)
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(navigation_bar())
        .with_flex_child(EditView::new().center(), 1.0)
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

    let demo_state = Workspace {
        input_text: "".into(),
    };

    let params = Params {
        debug_layout: false,
    };

    AppLauncher::with_window(main_window)
        .delegate(Delegate::default())
        .log_to_console()
        .launch(AppState {
            title: title.to_string(),
            workspace: demo_state,
            params,
        })
        .expect("Failed to launch application");
}
