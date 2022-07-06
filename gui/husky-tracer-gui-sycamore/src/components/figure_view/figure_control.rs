mod mutations;

use super::*;
use mutations::*;

#[derive(Prop)]
pub struct FigureControlProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureControl<'a, G: Html>(scope: Scope<'a>, props: FigureControlProps<'a>) -> View<G> {
    let tracer_context = use_context::<DebuggerContext>(scope);
    let opt_active_trace_id = &tracer_context.trace_context.opt_active_trace_id;
    let attention = &tracer_context.attention_context.attention;
    // let opt_data_and_control = memo!(
    //     scope,
    //     move || -> Option<(Rc<FigureCanvasData>, FigureControlData)> { None }
    // );
    let opt_canvas_and_control_data = memo!(scope, move || opt_active_trace_id.cget().map(
        |active_trace_id| {
            let active_trace = tracer_context.trace_context.trace_data(active_trace_id);
            let canvas_data = tracer_context
                .figure_context
                .figure_canvas_data(&active_trace, &attention.get());
            let control_data = tracer_context
                .figure_context
                .figure_control_data(&active_trace, &attention.get());
            (canvas_data, control_data)
        }
    ));
    view! {
        scope,
        (if let Some((canvas_data, figure_control_data)) = opt_canvas_and_control_data.cget() {
            match *canvas_data {
                FigureCanvasData::Mutations { ref mutations } => {
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
