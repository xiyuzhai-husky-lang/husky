use super::*;

#[derive(Prop)]
pub struct TraceTreeProps {
    trace_id: TraceId,
}

#[component]
pub fn TraceTree<'a, G: Html>(scope: Scope<'a>, props: TraceTreeProps) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let tree_context = &tracer_context.tree_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let expansion = tree_context.expanded_signal(props.trace_id);
    let associated_traces = tree_context.trace(props.trace_id).associated_traces();
    let focus = tracer_context.focus_context.focus_signal.clone();
    let subtraces = create_memo(scope, {
        move || tree_context.get_subtraces(&focus.get(), props.trace_id)
    });
    let child_trace_ids = create_memo(scope, {
        move || {
            let mut child_trace_ids = associated_traces.clone();
            if expansion.get_cloned() {
                child_trace_ids.extend(subtraces.get().iter())
            }
            child_trace_ids
        }
    });
    if shown.get_cloned() {
        view! {
            scope,
            div(class="TraceTree") {
                TraceNode {
                    trace_id: props.trace_id
                }
                Indexed {
                    iterable: child_trace_ids,
                    view: |scope, child_trace_id| view! {
                        scope,
                        TraceTree {
                            trace_id: child_trace_id,
                        }
                    },
                }
            }
        }
    } else {
        view! {scope, }
    }
}
