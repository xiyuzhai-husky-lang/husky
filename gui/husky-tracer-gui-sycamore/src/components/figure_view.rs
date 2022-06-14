mod figure_canvas;
mod figure_control;

use super::*;
use figure_canvas::*;
use figure_control::*;

#[derive(Prop, Clone)]
pub struct FigureViewProps<'a> {
    width: &'a ReadSignal<i32>,
    height: &'a ReadSignal<i32>,
}

impl<'a> FigureViewProps<'a> {
    fn canvas_dimension(&self) -> PixelDimension {
        log::info!("ad hoc");
        PixelDimension {
            width: self.width.get_cloned() * 4 / 5,
            height: self.height.get_cloned() * 97 * 98 / 10000,
        }
    }
    fn control_dimension(&self) -> PixelDimension {
        log::info!("ad hoc");
        let total_width = self.width.get_cloned();
        PixelDimension {
            width: total_width - total_width * 4 / 5,
            height: self.height.get_cloned() * 97 * 98 / 10000,
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
                ) {
                    FigureCanvas {
                        dimension: canvas_dimension
                    }
                }
                div (
                    class="FigureControlContainer",
                ) {
                    FigureControl {
                        dimension: control_dimension
                    }
                }
            }
        }
    }
}
