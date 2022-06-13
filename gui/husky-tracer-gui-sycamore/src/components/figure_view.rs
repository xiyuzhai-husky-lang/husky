mod figure_content;
mod figure_control;

use super::*;
use figure_content::*;
use figure_control::*;

#[derive(Prop, Clone)]
pub struct FigureViewProps {
    figure_content_data: FigureContentData,
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
        FigureContent {
            data: props.figure_content_data
        }
        FigureControl {
            data: props.figure_control_data
        }
    }
}
