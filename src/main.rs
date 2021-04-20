use druid::widget::prelude::*;
use druid::widget::{Flex, Label, SizedBox, TextBox, WidgetExt};
use druid::{AppLauncher, Color, Data, Lens, UnitPoint, WidgetId, WindowDesc};

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

/// builds a child Flex widget from some paramaters.
struct EditView {
    inner: Box<dyn Widget<Workspace>>,
}

impl EditView {
    fn new() -> EditView {
        EditView {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &Workspace) {
        let mut flex = Flex::row();

        flex.add_child(
            TextBox::multiline()
                .with_placeholder("Sample text")
                .with_text_color(Color::BLACK)
                .fix_width(400.0)
                .fix_height(600.0)
                .lens(ProjectState::input_text)
                .background(Color::WHITE),
        );

        let flex = flex
            .expand_width()
            .expand_height()
            .background(line::hline())
            .lens(Workspace::project);

        if data.params.debug_layout {
            self.inner = flex.debug_paint_layout().boxed()
        } else {
            self.inner = flex.boxed()
        }
    }
}

impl Widget<Workspace> for EditView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Workspace, env: &Env) {
        self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Workspace,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.rebuild_inner(data);
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Workspace, data: &Workspace, env: &Env) {
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
        data: &Workspace,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Workspace, env: &Env) {
        self.inner.paint(ctx, data, env)
    }

    fn id(&self) -> Option<WidgetId> {
        self.inner.id()
    }
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
