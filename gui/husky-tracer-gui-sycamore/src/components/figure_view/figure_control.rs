use super::*;

#[derive(Prop)]
pub struct FigureControlProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureControl<'a, G: Html>(scope: Scope<'a>, props: FigureControlProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let opt_active_trace_id = &context.tree_context.opt_active_trace_id;
    let focus = &context.focus_context.focus_signal;
    view! {
        scope,
        (if let Some(active_trace_id) = opt_active_trace_id.get_cloned() {
            let active_trace = context.tree_context.trace(active_trace_id);
            let canvas_data = memo!(
                scope,
                context
                    .figure_context
                    .figure_canvas_data(&active_trace, &focus.get()),
                active_trace
            );
            let control_data = memo!(
                scope,
                context
                    .figure_context
                    .figure_control_data(&active_trace, &focus.get())
            );
            match *canvas_data.get_cloned() {
                FigureCanvasData::Primitive { value } => todo!(),
                FigureCanvasData::Plot2d { plot_kind,ref point_groups, xrange, yrange } => todo!(),
                FigureCanvasData::Graphics2d { ref image_layers,ref shapes, xrange, yrange } => todo!(),
                FigureCanvasData::Mutations { ref mutations } => todo!(),
            }
        } else {
            "no active trace"
        })
    }
}
