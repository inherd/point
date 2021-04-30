use crate::app_state::AppState;
use crate::theme;
use druid::text::{Attribute, AttributeSpans, RichText};
use druid::{
    BoxConstraints, Color, Env, Event, EventCtx, FontFamily, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, TextLayout, UpdateCtx, Widget,
};
use druid_shell::piet::{PietTextLayoutBuilder, TextAttribute, TextLayoutBuilder};
use piet_common::Text;

pub struct EditView {}

impl EditView {
    pub fn new() -> EditView {
        EditView {}
    }
}

#[allow(dead_code)]
const TOP_PAD: f64 = 6.0;
const LEFT_PAD: f64 = 6.0;
const LINE_SPACE: f64 = 17.0;

impl Widget<AppState> for EditView {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    #[rustfmt::skip]
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppState, _env: &Env) {
    //
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env) {
        if old_data.theme_name != data.theme_name {
            ctx.request_paint();
        }
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

        let background = match &data.theme.background {
            None => Color::WHITE,
            Some(color) => theme::from_xi_color(color),
        };

        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &background);

        let text_color = match &data.theme.foreground {
            None => Color::BLACK,
            Some(color) => theme::from_xi_color(color),
        };

        for line in &data.workspace.line_cache.lines {
            if let Some(line) = line {
                let text = ctx.text();
                let mut layout = text.new_text_layout(line.text.clone());

                let mut end_index = 0usize;
                for style in &line.styles {
                    end_index = end_index + (style.offset as usize + style.length as usize);
                }

                for style in line.styles.iter().rev() {
                    let start_index = end_index - style.length as usize;
                    let line_style = data.styles.get(&(style.style_id as usize));

                    if let Some(foreground) = line_style.and_then(|s| s.fg_color) {
                        // let attr = TextAttribute::TextColor(theme::color_from_u32(foreground));
                        // layout.range_attribute(start_index..end_index, attr);
                    }
                }

                ctx.draw_text(&layout.build().unwrap(), (x0, y));
            }

            y += LINE_SPACE;
        }
    }
}
