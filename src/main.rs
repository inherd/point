pub mod print_ui;

use druid::widget::prelude::*;
use druid::widget::{Flex, Label, SizedBox, TextBox, WidgetExt};
use druid::{AppLauncher, Color, Data, Lens, Region, UnitPoint, WidgetId, WindowDesc};
use druid_shell::piet::Piet;
use druid_shell::{Application, WinHandler, WindowBuilder, WindowHandle};
use std::any::Any;

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
    spacer_size: f64,
}

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

fn build_widget(state: &Params) -> Box<dyn Widget<AppState>> {
    let mut flex = Flex::row();

    flex.add_child(
        TextBox::new()
            .with_placeholder("Sample text")
            .with_text_color(Color::WHITE)
            .lens(DemoState::input_text),
    );

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

struct UiMain {
    handle: WindowHandle,
}

impl WinHandler for UiMain {
    fn connect(&mut self, _handle: &WindowHandle) {}

    fn prepare_paint(&mut self) {}

    fn paint<'a>(&mut self, _piet: &mut Piet<'a>, invalid: &Region) {}

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let app = Application::new().unwrap();
    let mut builder = WindowBuilder::new(app.clone());
    let ui_main = UiMain {
        handle: Default::default(),
    };
    builder.set_handler(Box::new(ui_main));
    builder.set_title("Performance tester");

    let window = builder.build().unwrap();

    window.show();

    app.run(None);
}
