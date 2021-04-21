use crate::components::tree::TreeNode;
use crate::{theme, AppState};
use druid::kurbo::Line;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx,
    RenderContext, Size, UpdateCtx, Widget,
};
use std::fmt;
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

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: "".to_string(),
            icon: "".to_string(),
            path: Arc::new(Default::default()),
            children: vec![],
        }
    }
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

impl Data for FileEntry {
    fn same(&self, other: &Self) -> bool {
        self.name.same(&other.name)
            && *self.path == *self.path
            && self.children.len() == other.children.len()
            && self
                .children
                .iter()
                .zip(other.children.iter())
                .all(|(a, b)| a.same(b))
    }
}

impl TreeNode for FileEntry {
    fn children_count(&self) -> usize {
        self.children.len()
    }

    fn get_child(&self, index: usize) -> &Taxonomy {
        &self.children[index]
    }

    fn get_child_mut(&mut self, index: usize) -> &mut Taxonomy {
        &mut self.children[index]
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
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
