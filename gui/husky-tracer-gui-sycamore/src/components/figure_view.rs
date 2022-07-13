mod figure_content;
mod figure_control;

use super::*;
use figure_content::*;
use figure_control::*;

#[derive(Prop, Clone)]
pub struct FigureViewProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

const FIGURE_TOP_BAR_HEIGHT: u32 = 23;

impl<'a> FigureViewProps<'a> {
    fn content_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        PixelDimension {
            width: dimension.width - 4,
            height: dimension.height - FIGURE_TOP_BAR_HEIGHT,
        }
    }
    fn title_dimension(&self) -> PixelDimension {
        let dimension = self.dimension.cget();
        PixelDimension {
            width: dimension.width,
            height: FIGURE_TOP_BAR_HEIGHT,
        }
    }
}

#[component]
pub fn FigureView<'a, G: Html>(scope: Scope<'a>, props: FigureViewProps<'a>) -> View<G> {
    let dimension = props.dimension;
    let content_dimension = memo!(scope, move || props.content_dimension(), props);
    let title_dimension = memo!(scope, move || props.title_dimension(), props);
    view! {
        scope,
        div (class="FigureView disable-select") {
            div (
                class="FigureTitle",
                style=title_dimension.cget().to_style(),
            ) {
                label { "title" }
            }
            div (
                class="FigureContent",
                style=dimension.cget().to_style(),
            ) {
                FigureContent {
                    dimension: content_dimension
                }
            }
        }
    }
}
