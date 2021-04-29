use crate::app_state::AppState;
use crate::linecache::Line;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size,
    UpdateCtx, Widget,
};

pub struct EditView {}

impl EditView {
    pub fn new() -> EditView {
        EditView {}
    }
}

impl Widget<AppState> for EditView {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    #[rustfmt::skip]
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        let max_width = bc.max().width;
        let max_height = bc.max().height;
        bc.constrain(Size::new(max_width, max_height))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        for line in &data.workspace.line_cache.lines {
            match line {
                None => {}
                Some(line) => {
                    println!("{:?}", line.text);
                }
            }
        }
    }
}
