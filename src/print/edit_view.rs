use druid::widget::{Flex, SizedBox, TextBox};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetId,
};

use crate::app_state::{AppState, Workspace};

pub struct EditView {
    inner: Box<dyn Widget<AppState>>,
}

impl EditView {
    pub fn new() -> EditView {
        EditView {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        let mut flex = Flex::column();

        flex.add_flex_child(
            TextBox::multiline()
                .with_text_color(Color::BLACK)
                // .with_font(crate::theme::WRITING_FONT)
                .expand_width()
                .expand_height()
                .lens(Workspace::input_text)
                .background(Color::WHITE),
            1.0,
        );

        let flex = flex
            .expand_width()
            .expand_height()
            .lens(AppState::workspace);

        if data.params.debug_layout {
            self.inner = flex.debug_paint_layout().boxed()
        } else {
            self.inner = flex.boxed()
        }
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
