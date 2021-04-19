pub mod print_ui;

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, SizedBox, TextBox, WidgetExt};
use druid::{
    commands, platform_menus, AppLauncher, Color, Data, FileDialogOptions, Lens, LocalizedString,
    MenuDesc, MenuItem, SysMods, UnitPoint, WidgetId, WindowDesc,
};

const DEFAULT_SPACER_SIZE: f64 = 8.;
const LIGHTER_GREY: Color = Color::rgb8(242, 242, 242);

#[derive(Clone, Data, Lens)]
struct AppState {
    title: String,
    demo_state: DemoState,
    params: Params,
}

#[derive(Clone, Data, Lens)]
struct DemoState {
    pub input_text: String,
    pub enabled: bool,
    volume: f64,
}

#[derive(Clone, Data, Lens)]
struct Params {
    debug_layout: bool,
    spacers: Spacers,
    spacer_size: f64,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Data)]
enum Spacers {
    None,
    Default,
    Flex,
    Fixed,
}

#[derive(Clone, Copy, PartialEq, Data)]
#[allow(dead_code)]
enum FlexType {
    Row,
    Column,
}

/// builds a child Flex widget from some paramaters.
struct EditView {
    inner: Box<dyn Widget<AppState>>,
}

impl EditView {
    fn new() -> EditView {
        EditView {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        self.inner = build_widget(&data.params);
    }
}

impl Widget<AppState> for EditView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        if !old_data.params.same(&data.params) {
            self.rebuild_inner(data);
            ctx.children_changed();
        } else {
            self.inner.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
}

fn navigation_bar() -> impl Widget<AppState> {
    Flex::row()
        .with_child(Label::new("print/src/main.rs").with_text_color(Color::BLACK))
        .padding(10.0)
        .border(Color::grey(0.6), 1.0)
        .expand_width()
        .lens(AppState::params)
        .align_horizontal(UnitPoint::LEFT)
}

fn space_if_needed<T: Data>(flex: &mut Flex<T>, params: &Params) {
    match params.spacers {
        Spacers::None => (),
        Spacers::Default => flex.add_default_spacer(),
        Spacers::Fixed => flex.add_spacer(params.spacer_size),
        Spacers::Flex => flex.add_flex_spacer(1.0),
    }
}

fn build_widget(state: &Params) -> Box<dyn Widget<AppState>> {
    let mut flex = Flex::row();

    flex.add_child(
        TextBox::new()
            .with_placeholder("Sample text")
            .with_text_color(Color::WHITE)
            .lens(DemoState::input_text),
    );
    space_if_needed(&mut flex, state);

    let flex = flex
        .lens(AppState::demo_state)
        .background(Color::WHITE)
        .expand_width()
        .expand_height();

    if state.debug_layout {
        flex.debug_paint_layout().boxed()
    } else {
        flex.boxed()
    }
}

fn status_bar() -> impl Widget<AppState> {
    Flex::row()
        .with_default_spacer()
        .with_flex_child(Label::new("status bar").with_text_color(Color::BLACK), 1.0)
        .with_default_spacer()
        .with_flex_child(Label::new("time").with_text_color(Color::BLACK), 1.0)
        .lens(AppState::params)
        .padding(5.0)
        .border(Color::grey(0.6), 1.0)
        .expand_width()
        .align_horizontal(UnitPoint::LEFT)
}

fn make_ui() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(navigation_bar())
        .with_default_spacer()
        .with_flex_child(EditView::new().center(), 1.0)
        .with_default_spacer()
        .with_child(status_bar())
        .background(LIGHTER_GREY)
}

fn menus<T: Data>() -> MenuDesc<T> {
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

pub fn main() {
    let title = "Print UI";

    let menu = menus();

    let main_window = WindowDesc::new(make_ui)
        .window_size((720., 600.))
        .with_min_size((620., 300.))
        .menu(menu)
        .title(title);

    let demo_state = DemoState {
        input_text: "hello".into(),
        enabled: false,
        volume: 0.0,
    };

    let params = Params {
        debug_layout: false,
        spacers: Spacers::None,
        spacer_size: DEFAULT_SPACER_SIZE,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(AppState {
            title: title.to_string(),
            demo_state,
            params,
        })
        .expect("Failed to launch application");
}
