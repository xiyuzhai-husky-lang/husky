use super::*;

#[derive(Prop, Clone)]
pub struct HSplitPanelProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

const LEFT_PANEL_PERCENTAGE: u32 = 50;

impl<'a> HSplitPanelProps<'a> {
    fn left_panel_dimension(&self) -> PixelDimension {
        self.dimension.cget() * (LEFT_PANEL_PERCENTAGE, 1) / (100, 1)
    }

    fn right_panel_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        let left_panel_width = self.left_panel_dimension().width;
        PixelDimension {
            width: dimension.width - left_panel_width,
            height: dimension.height,
        }
    }
}

#[component]
pub fn HSplitPanel<'a, G: Html>(scope: Scope<'a>, props: HSplitPanelProps<'a>) -> View<G> {
    let context = use_context::<DebuggerContext>(scope);
    let root_trace_ids = &context.trace_context.root_trace_ids;
    let left_panel_dimension = memo!(scope, move || props.left_panel_dimension(), props);
    let right_panel_dimension = memo!(scope, move || props.right_panel_dimension(), props);
    view! {
        scope,
        div(class="HuskyTracerHSplitPanel") {
            div(class="HuskyTracerHSplitPanelLeft", style=left_panel_dimension.cget().to_style()) {
                TraceView {}
            }
            div(class="HuskyTracerHSplitPanelRight", style=right_panel_dimension.cget().to_style()) {
                FigureView {
                    dimension: right_panel_dimension
                }
            }
        }
    }
}
