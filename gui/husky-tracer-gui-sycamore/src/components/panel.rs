use super::*;

#[derive(Prop)]
pub struct HSplitPanelProps<'a> {
    width: &'a ReadSignal<f64>,
    height: &'a ReadSignal<f64>,
}

impl<'a> HSplitPanelProps<'a> {
    fn left_panel_style(&self) -> String {
        let left_panel_width = self.left_panel_width();
        let left_panel_height = self.panel_height();
        format!("width: {left_panel_width}px; height: {left_panel_height}px")
    }

    fn left_panel_width(&self) -> f64 {
        self.width.get_cloned() / 2.
    }

    fn panel_height(&self) -> f64 {
        self.height.get_cloned()
    }
}

#[component]
pub fn HSplitPanel<'a, G: Html>(scope: Scope<'a>, props: HSplitPanelProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let root_trace_ids = &context.tree_context.root_trace_ids;
    // create_effect(scope, move || {
    //     log::info!("root traces {:?}", root_trace_ids)
    // });
    props.width.track();
    let right_style = "width: 800px";
    view! {
        scope,
        div(class="HuskyTracerHSplitPanel") {
            div(class="HuskyTracerHSplitPanelLeft", style=props.left_panel_style()) {
                TraceView {}
            }
            // div(class="HuskyTracerHSplitPanelRight", style=right_style) {
            //     "Value: " (props.width.get())
            // }
        }
    }
}
