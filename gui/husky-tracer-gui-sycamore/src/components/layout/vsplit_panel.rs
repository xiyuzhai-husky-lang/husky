use super::hsplit_panel::*;
use super::*;

#[derive(Prop, Clone)]
pub struct VSplitPanelProps<'a> {
    width: &'a ReadSignal<f64>,
    height: &'a ReadSignal<f64>,
}

impl<'a> VSplitPanelProps<'a> {
    fn upper_panel_style(&self) -> String {
        let upper_panel_width = self.panel_width();
        let upper_panel_height = self.upper_panel_height();
        format!("width: {upper_panel_width}px; height: {upper_panel_height}px")
    }

    fn panel_width(&self) -> f64 {
        self.width.get_cloned()
    }

    fn upper_panel_height(&self) -> f64 {
        self.height.get_cloned() - self.lower_panel_height()
    }

    fn lower_panel_height(&self) -> f64 {
        35.
    }
}

#[component]
pub fn VSplitPanel<'a, G: Html>(scope: Scope<'a>, props: VSplitPanelProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let root_trace_ids = &context.tree_context.root_trace_ids;
    // create_effect(scope, move || {
    //     log::info!("root traces {:?}", root_trace_ids)
    // });
    let upper_panel_style = create_memo(scope, {
        let props = props.clone();
        move || props.upper_panel_style()
    });
    let upper_panel_height = create_memo(scope, {
        let props = props.clone();
        move || props.upper_panel_height()
    });
    let panel_width = create_memo(scope, {
        let props = props.clone();
        move || props.panel_width()
    });
    props.width.track();
    let right_style = "width: 800px";
    view! {
        scope,
        div(class="HuskyTracerVSplitPanel") {
            div(class="HuskyTracerVSplitPanelUpper", style=upper_panel_style) {
                HSplitPanel {
                    height: upper_panel_height,
                    width:  panel_width,
                }
            }
            div(class="HuskyTracerVSplitPanelLower", style=right_style) {
                "Value: " (props.width.get())
            }
        }
    }
}
