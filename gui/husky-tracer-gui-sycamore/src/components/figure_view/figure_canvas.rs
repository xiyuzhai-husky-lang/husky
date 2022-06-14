mod graphics2d;
mod plot2d;

use super::*;
use graphics2d::*;
use plot2d::*;

#[derive(Prop)]
pub struct FigureCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn FigureCanvas<'a, G: Html>(scope: Scope<'a>, props: FigureCanvasProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let opt_active_trace_id = &context.tree_context.opt_active_trace_id;
    let focus = &context.focus_context.focus_signal;
    if let Some(active_trace_id) = opt_active_trace_id.get_cloned() {
        let active_trace = context.tree_context.trace(active_trace_id);
        let data = memo!(
            scope,
            context
                .figure_context
                .figure_canvas_data(&active_trace, &focus.get())
        );
        match *data.get_cloned() {
            FigureCanvasData::Primitive { value } => todo!(),
            FigureCanvasData::Plot2d { .. } => todo!(),
            FigureCanvasData::Graphics2d { .. } => todo!(),
            FigureCanvasData::Mutations { .. } => todo!(),
        }
    } else {
        view! {scope,}
    }
}
