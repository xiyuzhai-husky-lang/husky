mod figure_canvas;
mod figure_control;

use super::*;
use figure_canvas::*;
use figure_control::*;

#[component]
pub fn FigureView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    view! {
        scope,
        div (class="FigureView disable-select") {
            p {
                "title"
            }
        }
        FigureCanvas {
        }
        FigureControl {
        }
    }
}
