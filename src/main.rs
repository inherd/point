use druid::widget::prelude::*;
use druid::widget::{Flex, Label, WidgetExt};
use druid::{AppLauncher, Color, Data, Lens, UnitPoint, WindowDesc};

use print_ui::editor::EditView;

pub mod line;
pub mod menu;
pub mod print_ui;
pub mod theme;

const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

#[derive(Clone, Data, Lens)]
struct Workspace {
    title: String,
    project: ProjectState,
    params: Params,
}

#[derive(Clone, Data, Lens)]
struct ProjectState {
    pub input_text: String,
}

#[derive(Clone, Data, Lens)]
struct Params {
    debug_layout: bool,
}

fn navigation_bar() -> impl Widget<Workspace> {
    Flex::row()
        .with_child(Label::new("print/src/main.rs").with_text_color(Color::BLACK))
        .padding(10.0)
        .expand_width()
        .lens(Workspace::params)
        .background(line::hline())
        .align_horizontal(UnitPoint::LEFT)
}

fn status_bar() -> impl Widget<Workspace> {
    Flex::row()
        .with_default_spacer()
        .with_flex_child(Label::new("status bar").with_text_color(Color::BLACK), 1.0)
        .with_default_spacer()
        .with_flex_child(Label::new("time").with_text_color(Color::BLACK), 1.0)
        .lens(Workspace::params)
        .padding(5.0)
        .align_horizontal(UnitPoint::LEFT)
}

fn make_ui() -> impl Widget<Workspace> {
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

    let demo_state = ProjectState {
        input_text: "".into(),
    };

    let params = Params {
        debug_layout: false,
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(Workspace {
            title: title.to_string(),
            project: demo_state,
            params,
        })
        .expect("Failed to launch application");
}
