use crate::*;

#[derive(Prop)]
pub struct TraceNodeProps {
    trace_id: TraceId,
}

#[component]
pub fn TraceNode<'a, G: Html>(scope: Scope<'a>, props: TraceNodeProps) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let tree_context = &tracer_context.tree_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let expansion = tree_context.expanded_signal(props.trace_id);
    todo!()
}
