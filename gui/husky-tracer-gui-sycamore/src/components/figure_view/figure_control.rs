mod mutations;

use super::*;
use mutations::*;

#[derive(Prop)]
pub struct FigureControlProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureControl<'a, G: Html>(scope: Scope<'a>, props: FigureControlProps<'a>) -> View<G> {
    let ctx = use_dev_context(scope);
    let presentation_signal = ctx.presentation_signal();
    let opt_canvas_and_control_data = memo!(scope, move || {
        presentation_signal
            .get()
            .opt_active_trace_id()
            .map(|active_trace_id| {
                let active_trace = ctx.trace_data(active_trace_id);
                let canvas_data = ctx.figure_canvas_data(&active_trace, &presentation_signal.get());
                let control_data =
                    ctx.figure_control_data(&active_trace, &presentation_signal.get());
                (canvas_data, control_data)
            })
    });
    view! {
        scope,
        (if let Some((canvas_data, figure_control_data)) = opt_canvas_and_control_data.cget() {
            match *canvas_data {
                FigureCanvasData::Specific(SpecificFigureCanvasData::Mutations { ref mutations }) => {
                    view! {
                        scope,
                        MutationsControl {
                            mutations,
                            figure_control_data,
                            dimension: props.dimension,
                        }
                    }
                },
                _ => view! {scope, }
            }
        } else {
            view! {scope, "no active trace"}
        })
    }
}
