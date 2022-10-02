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
    let opt_active_trace_id = &ctx.trace_context.opt_active_trace_id;
    let restriction = &ctx.restriction_context.restriction;
    let opt_canvas_and_control_data = memo!(scope, move || opt_active_trace_id.cget().map(
        |active_trace_id| {
            let active_trace = ctx.trace_context.trace_data(active_trace_id);
            let canvas_value = ctx.figure_canvas_data(&active_trace, &restriction.get());
            let control_data = ctx.figure_control_data(&active_trace, &restriction.get());
            (canvas_value, control_data)
        }
    ));
    view! {
        scope,
        (if let Some((canvas_value, figure_control_data)) = opt_canvas_and_control_data.cget() {
            match *canvas_value {
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
