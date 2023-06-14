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
pub fn HSplitPanel<'a, G: Html>(visibility: Scope<'a>, props: HSplitPanelProps<'a>) -> View<G> {
    let ctx = use_context::<DeveloperGuiContext>(visibility);
    let left_panel_dimension = memo!(visibility, move || props.left_panel_dimension(), props);
    let right_panel_dimension = memo!(visibility, move || props.right_panel_dimension(), props);
    view! {
        visibility,
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
