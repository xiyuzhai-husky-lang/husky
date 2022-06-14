mod figure_canvas;
mod figure_control;

use super::*;
use figure_canvas::*;
use figure_control::*;

#[derive(Prop, Clone)]
pub struct FigureViewProps {
    figure_canvas_data: FigureCanvasData,
    figure_control_data: FigureControlData,
}

#[component]
pub fn FigureView<'a, G: Html>(scope: Scope<'a>, props: FigureViewProps) -> View<G> {
    view! {
        scope,
        div (class="FigureView disable-select") {
            p {
                "title"
            }
        }
        FigureCanvas {
            data: props.figure_canvas_data
        }
        FigureControl {
            data: props.figure_control_data
        }
    }
}
