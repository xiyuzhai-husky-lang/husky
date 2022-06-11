use crate::*;

#[derive(Prop)]
pub struct TraceTreeProps {
    trace_id: TraceId,
}

#[component]
pub fn TraceTree<'a, G: Html>(scope: Scope<'a>, props: TraceTreeProps) -> View<G> {
    let tree_context = &use_context::<TracerContext>(scope).tree_context;
    let trace = tree_context.trace(props.trace_id);
    view! {
        scope,
        div(class="TraceTree") {
        }
    }
}
