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
            width: 500,
            height: 500,
        }
    }
    fn control_dimension(&self) -> PixelDimension {
        log::info!("ad hoc");
        PixelDimension {
            width: 500,
            height: 100,
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
                style="flex-direction: column"
            ) {
                div (
                    class="FigureCanvasContainer",
                    style=canvas_dimension.get().to_style()
                ) {
                    FigureCanvas {
                        dimension: canvas_dimension
                    }
                }
                div (
                    class="FigureControlContainer",
                    style=control_dimension.get().to_style()
                ) {
                    FigureControl {
                        dimension: control_dimension
                    }
                }
            }
        }
    }
}
