mod mutations;

use super::*;
use mutations::*;

#[derive(Prop)]
pub struct FigureControlProps<'a> {
    figure_canvas_value: &'a ReadSignal<FigureCanvasValue>,
    dimension: &'a ReadSignal<PixelDimension>,
}

pub fn FigureControl<'a, G: Html>(scope: Scope<'a>, props: FigureControlProps<'a>) -> View<G> {
    let ctx = use_dev_context(scope);
    let presentation_signal = ctx.presentation_signal();
    let opt_control_data = memo!(scope, move || {
        presentation_signal
            .get()
            .opt_active_trace_id()
            .map(|active_trace_id| {
                let active_trace = ctx.trace_data(active_trace_id);
                ctx.figure_control_data(&active_trace, &presentation_signal.get())
            })
    });
    view! {
        scope,
        (if let Some(figure_control_data) = opt_control_data.cget() {
            match *figure_control_data.get() {
                FigureControlData::Unit => view! {scope, },
                FigureControlData::Mutations { opt_mutation_selection } =>
                view! {scope,
                MutationsControl {
                    mutations: todo!(),
                    figure_control_data: todo!(),
                    dimension: props.dimension,
                }}
            }
        } else {
            view! {scope, "no active trace"}
        })
    }
}
