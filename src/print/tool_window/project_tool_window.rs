use crate::{theme, AppState};
use druid::kurbo::Line;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, Size, UpdateCtx, Widget,
};
use std::path::PathBuf;
use std::sync::Arc;

pub struct ProjectToolWindow {}

#[derive(Clone, Lens)]
struct FileEntry {
    pub name: String,
    pub icon: String,
    pub path: Arc<PathBuf>,
    pub children: Vec<FileEntry>,
}

impl FileEntry {
    pub fn new(name: &'static str, path: &PathBuf) -> Self {
        FileEntry {
            name: name.to_string(),
            icon: "".to_string(),
            path: Arc::new(path.to_owned()),
            children: vec![],
        }
    }

    pub fn add_child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }
}

impl ProjectToolWindow {
    pub fn new() -> ProjectToolWindow {
        ProjectToolWindow {}
    }

    pub fn draw() {}
}

#[allow(unused_variables)]
impl Widget<AppState> for ProjectToolWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, env: &Env) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        let size = 32.0;
        bc.constrain(Size::new(size, f64::INFINITY))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let rect = ctx.size().to_rect();
        let x_pos = rect.width() - 0.5;
        let line = Line::new((x_pos, 0.0), (x_pos, rect.height()));

        ctx.fill(rect, &env.get(theme::TOOL_WINDOW_COLOR));
        ctx.stroke(line, &env.get(theme::SIDEBAR_EDGE_STROKE), 1.0);
    }
}
