use super::*;

#[component]
pub fn FigureControl<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let opt_active_trace_id = &context.tree_context.opt_active_trace_id;
    let focus = &context.focus_context.focus_signal;
    if let Some(active_trace_id) = opt_active_trace_id.get_cloned() {
        todo!()
    } else {
        view! {scope,}
    }
}
