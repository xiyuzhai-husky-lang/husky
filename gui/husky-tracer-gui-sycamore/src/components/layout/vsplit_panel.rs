use super::hsplit_panel::*;
use super::*;

#[derive(Prop, Clone)]
pub struct VSplitPanelProps<'a> {
    width: &'a ReadSignal<u32>,
    height: &'a ReadSignal<u32>,
}

impl<'a> VSplitPanelProps<'a> {
    fn upper_panel_style(&self) -> String {
        let upper_panel_width = self.panel_width();
        let upper_panel_height = self.upper_panel_height();
        format!("width: {upper_panel_width}px; height: {upper_panel_height}px")
    }
    fn lower_panel_style(&self) -> String {
        let lower_panel_width = self.panel_width();
        let lower_panel_height = self.lower_panel_height();
        format!("width: {lower_panel_width}px; height: {lower_panel_height}px")
    }

    fn panel_width(&self) -> u32 {
        self.width.get_cloned()
    }

    fn upper_panel_height(&self) -> u32 {
        self.height.get_cloned() - self.lower_panel_height()
    }

    fn lower_panel_height(&self) -> u32 {
        23
    }
}

#[component]
pub fn VSplitPanel<'a, G: Html>(scope: Scope<'a>, props: VSplitPanelProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let root_trace_ids = &context.tree_context.root_trace_ids;
    let upper_panel_style = memo!(scope, props.upper_panel_style(), props);
    let upper_panel_height = memo!(scope, props.upper_panel_height(), props);
    let lower_panel_style = memo!(scope, props.lower_panel_style(), props);
    let lower_panel_height = memo!(scope, props.lower_panel_height(), props);
    let panel_width = memo!(scope, props.panel_width(), props);
    view! {
        scope,
        div(class="HuskyTracerVSplitPanel") {
            div(class="HuskyTracerVSplitPanelUpper", style=upper_panel_style) {
                HSplitPanel {
                    height: upper_panel_height,
                    width:  panel_width,
                }
            }
            div(class="HuskyTracerVSplitPanelLower", style=lower_panel_style) {
                FocusView {}
            }
        }
    }
}
