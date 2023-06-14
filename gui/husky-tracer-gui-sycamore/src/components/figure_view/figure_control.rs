mod mutations;

use super::*;
use mutations::*;

#[derive(Prop)]
pub struct FigureControlProps<'a> {
    figure_canvas_value: &'a ReadSignal<FigureCanvasValue>,
    dimension: &'a ReadSignal<PixelDimension>,
    opt_control_data: &'a ReadSignal<Option<&'a ReadSignal<FigureControlData>>>,
}

pub fn FigureControl<'a, G: Html>(visibility: Scope<'a>, props: FigureControlProps<'a>) -> View<G> {
    let ctx = use_dev_context(visibility);
    let presentation_signal = ctx.presentation_signal();
    view! {
        visibility,
        (if let Some(figure_control_data) = props.opt_control_data.cget() {
            match *figure_control_data.get() {
                FigureControlData::Unit => view! {visibility, },
                FigureControlData::Mutations { opt_mutation_selection } =>
                view! {visibility,
                MutationsControl {
                    mutations: todo!(),
                    figure_control_data: todo!(),
                    dimension: props.dimension,
                }}
            }
        } else {
            view! {visibility, "no active trace"}
        })
    }
}
