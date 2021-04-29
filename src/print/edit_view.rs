use crate::app_state::AppState;
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, FontFamily, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, UpdateCtx, Widget,
};
use druid_shell::piet::TextLayoutBuilder;
use piet_common::Text;

pub struct EditView {}

impl EditView {
    pub fn new() -> EditView {
        EditView {}
    }
}

const TOP_PAD: f64 = 6.0;
const LEFT_PAD: f64 = 6.0;
const LINE_SPACE: f64 = 17.0;

impl Widget<AppState> for EditView {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    #[rustfmt::skip]
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {
    //     
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
        let x0 = LEFT_PAD;
        let mut y: f64 = 0.0;

        let first_line: u64 = 0;
        let last_line = data.workspace.line_cache.height();
        for _line_num in first_line..last_line {
            //
        }

        for line in &data.workspace.line_cache.lines {
            if let Some(line) = line {
                let text = ctx.text();
                let layout = text
                    .new_text_layout(line.text.clone())
                    .font(FontFamily::SERIF, 16.0)
                    .text_color(Color::rgb8(128, 0, 0))
                    .build()
                    .unwrap();

                ctx.draw_text(&layout, (x0, y));
            }

            y += LINE_SPACE;
        }
    }
}
