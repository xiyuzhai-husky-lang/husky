mod figure_canvas;
mod figure_control;

use super::*;
use figure_canvas::*;
use figure_control::*;

#[derive(Prop, Clone)]
pub struct FigureViewProps<'a> {
    width: &'a ReadSignal<u32>,
    height: &'a ReadSignal<u32>,
}

impl<'a> FigureViewProps<'a> {
    fn canvas_dimension(&self) -> PixelDimension {
        PixelDimension {
            width: self.width.get_cloned() * 95 / 100 * 4 / 5,
            height: self.height.get_cloned() * 97 * 95 / 10000,
        }
    }
    fn control_dimension(&self) -> PixelDimension {
        let total_width = self.width.get_cloned();
        PixelDimension {
            width: total_width * 95 / 100 - total_width * 95 / 100 * 4 / 5,
            height: self.height.get_cloned() * 97 * 95 / 10000,
        }
    }
}

#[component]
pub fn FigureView<'a, G: Html>(scope: Scope<'a>, props: FigureViewProps<'a>) -> View<G> {
    let canvas_dimension = memo!(scope, props.canvas_dimension(), props);
    let control_dimension = memo!(scope, props.control_dimension(), props);
    view! {
        scope,
        div (class="FigureView disable-select") {
            p {
                "title"
            }
            div (
                class="FigureContent",
                style="flex-direction: row"
            ) {
                div (
                    class="FigureCanvasContainer",
                    style=canvas_dimension.get().to_style(),
                ) {
                    FigureCanvas {
                        dimension: canvas_dimension
                    }
                }
                div (
                    class="FigureControlContainer",
                    style=control_dimension.get().to_style(),
                ) {
                    FigureControl {
                        dimension: control_dimension
                    }
                }
            }
        }
    }
}
