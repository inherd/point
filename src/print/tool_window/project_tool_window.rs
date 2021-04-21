use crate::components::tree::{Tree, TreeNode};
use crate::AppState;
use druid::widget::{Flex, Label, Scroll, SizedBox};
use druid::{
    BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WidgetId,
};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Lens)]
pub struct FileEntry {
    pub name: String,
    pub icon: String,
    pub children: Vec<FileEntry>,
}

impl Default for FileEntry {
    fn default() -> Self {
        FileEntry {
            name: "".to_string(),
            icon: "".to_string(),
            children: vec![],
        }
    }
}

impl FileEntry {
    pub fn new(name: String) -> Self {
        FileEntry {
            name: name,
            icon: "".to_string(),
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

    fn get_child(&self, index: usize) -> &FileEntry {
        &self.children[index]
    }

    fn get_child_mut(&mut self, index: usize) -> &mut FileEntry {
        &mut self.children[index]
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

pub struct ProjectToolWindow {
    inner: Box<dyn Widget<AppState>>,
}

impl ProjectToolWindow {
    pub fn new() -> ProjectToolWindow {
        ProjectToolWindow {
            inner: SizedBox::empty().boxed(),
        }
    }

    fn rebuild_inner(&mut self, data: &AppState) {
        let mut flex = Flex::row();

        if data.workspace.current_dir.is_some() {
            let scroll = Scroll::new(Tree::new(|t: &FileEntry| Label::new(t.name.as_str())));
            flex.add_child(scroll);
        }

        let flex = flex.lens(AppState::entry);

        self.inner = flex.debug_paint_layout().boxed()
    }
}

#[allow(unused_variables)]
impl Widget<AppState> for ProjectToolWindow {
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
        if !old_data
            .workspace
            .current_dir
            .same(&data.workspace.current_dir)
        {
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
        self.inner.paint(ctx, data, env);
        //
        // let rect = ctx.size().to_rect();
        // let x_pos = rect.width() - 0.5;
        // let line = Line::new((x_pos, 0.0), (x_pos, rect.height()));
        //
        // ctx.fill(rect, &env.get(theme::TOOL_WINDOW_COLOR));
        // ctx.stroke(line, &env.get(theme::SIDEBAR_EDGE_STROKE), 1.0);
    }
}
