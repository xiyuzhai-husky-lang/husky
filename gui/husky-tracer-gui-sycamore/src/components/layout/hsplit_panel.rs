use super::*;

#[derive(Prop, Clone)]
pub struct HSplitPanelProps<'a> {
    width: &'a ReadSignal<i32>,
    height: &'a ReadSignal<i32>,
}

impl<'a> HSplitPanelProps<'a> {
    fn left_panel_style(&self) -> String {
        let left_panel_width = self.left_panel_width();
        let left_panel_height = self.panel_height();
        format!("width: {left_panel_width}px; height: {left_panel_height}px")
    }

    fn left_panel_width(&self) -> i32 {
        self.width.get_cloned() / 2
    }

    fn right_panel_style(&self) -> String {
        let right_panel_width = self.right_panel_width();
        let right_panel_height = self.panel_height();
        format!("width: {right_panel_width}px; height: {right_panel_height}px")
    }

    fn right_panel_width(&self) -> i32 {
        self.width.get_cloned() - self.left_panel_width()
    }

    fn panel_height(&self) -> i32 {
        self.height.get_cloned()
    }
}

#[component]
pub fn HSplitPanel<'a, G: Html>(scope: Scope<'a>, props: HSplitPanelProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let root_trace_ids = &context.tree_context.root_trace_ids;
    let left_panel_style = memo!(scope, props.left_panel_style(), props);
    let right_panel_style = memo!(scope, props.right_panel_style(), props);
    view! {
        scope,
        div(class="HuskyTracerHSplitPanel") {
            div(class="HuskyTracerHSplitPanelLeft", style=left_panel_style) {
                TraceView {}
            }
            div(class="HuskyTracerHSplitPanelRight", style=right_panel_style) {
                FigureView {}
            }
        }
    }
}
