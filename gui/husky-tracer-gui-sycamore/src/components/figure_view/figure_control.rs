mod mutations;

use super::*;

#[derive(Prop)]
pub struct FigureControlProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureControl<'a, G: Html>(scope: Scope<'a>, props: FigureControlProps<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let opt_active_trace_id = &tracer_context.tree_context.opt_active_trace_id;
    let focus = &tracer_context.focus_context.focus;
    // let opt_data_and_control = memo!(
    //     scope,
    //     move || -> Option<(Rc<FigureCanvasData>, FigureControlData)> { None }
    // );
    let opt_data_and_control = memo!(scope, move || opt_active_trace_id.cget().map(
        |active_trace_id| {
            let active_trace = tracer_context.tree_context.trace(active_trace_id);
            let canvas_data = tracer_context
                .figure_context
                .figure_canvas_data(&active_trace, &focus.get());
            let control_data = tracer_context
                .figure_context
                .figure_control_data(&active_trace, &focus.get());
            (canvas_data, control_data)
        }
    ));
    view! {
        scope,
        (if let Some((canvas_data,control_data)) = opt_data_and_control.cget() {
            match *canvas_data {
                FigureCanvasData::Mutations { ref mutations } => todo!(),
                _=> view! {scope, }
            }
        } else {
            view! {scope, "no active trace"}
        })
    }
}
