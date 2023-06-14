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
pub fn VSplitPanel<'a, G: Html>(visibility: Scope<'a>, props: VSplitPanelProps<'a>) -> View<G> {
    let ctx = use_dev_context(visibility);
    let root_trace_ids = &ctx.root_trace_ids_signal();
    let upper_panel_dimension = memo!(visibility, move || props.upper_panel_dimension(), props);
    let lower_panel_dimension = memo!(visibility, move || props.lower_panel_dimension(), props);
    view! {
        visibility,
        div(class="HuskyTracerVSplitPanel") {
            div(class="HuskyTracerVSplitPanelUpper", style=upper_panel_dimension.cget().to_style()) {
                HSplitPanel {
                    dimension: upper_panel_dimension
                }
            }
            div(class="HuskyTracerVSplitPanelLower", style=lower_panel_dimension.cget().to_style()) {
                PresentationView {}
            }
        }
    }
}
