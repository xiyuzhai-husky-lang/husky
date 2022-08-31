use super::hsplit_panel::*;
use super::*;

#[derive(Prop, Clone)]
pub struct VSplitPanelProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

const LOWER_PANEL_HEIGHT: u32 = 23;

impl<'a> VSplitPanelProps<'a> {
    fn upper_panel_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        PixelDimension {
            width: dimension.width,
            height: dimension.height - LOWER_PANEL_HEIGHT,
        }
    }

    fn lower_panel_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        PixelDimension {
            width: dimension.width,
            height: LOWER_PANEL_HEIGHT,
        }
    }
}

#[component]
pub fn VSplitPanel<'a, G: Html>(scope: Scope<'a>, props: VSplitPanelProps<'a>) -> View<G> {
    let context = use_debugger_context(scope);
    let root_trace_ids = &context.trace_context.root_trace_ids;
    let upper_panel_dimension = memo!(scope, move || props.upper_panel_dimension(), props);
    let lower_panel_dimension = memo!(scope, move || props.lower_panel_dimension(), props);
    view! {
        scope,
        div(class="HuskyTracerVSplitPanel") {
            div(class="HuskyTracerVSplitPanelUpper", style=upper_panel_dimension.cget().to_style()) {
                HSplitPanel {
                    dimension: upper_panel_dimension
                }
            }
            div(class="HuskyTracerVSplitPanelLower", style=lower_panel_dimension.cget().to_style()) {
                RestrictionView {}
            }
        }
    }
}
