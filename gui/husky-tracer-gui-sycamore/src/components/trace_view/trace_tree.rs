use super::*;

#[derive(Prop)]
pub struct TraceTreeProps {
    trace_id: TraceId,
}

#[component]
pub fn TraceTree<'a, G: Html>(scope: Scope<'a>, props: TraceTreeProps) -> View<G> {
    let tracer_context = use_context::<DebuggerContext>(scope);
    let tree_context = &tracer_context.trace_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let attention = tracer_context.attention_context.attention.clone();
    let attention = memo!(scope, move || attention.cget());
    let trace = tree_context.trace(props.trace_id);
    let associated_trace_trees = View::new_fragment(
        trace
            .associated_trace_ids()
            .into_iter()
            .map(|trace_id| {
                view! { scope, TraceTree {
                    trace_id
                } }
            })
            .collect(),
    );
    let subtrace_ids = memo!(scope, {
        let expansion = tree_context.expanded_signal(props.trace_id);
        move || {
            if expansion.cget() {
                tree_context
                    .subtrace_ids(&attention.get(), props.trace_id)
                    .to_vec()
            } else {
                vec![]
            }
        }
    });
    view! {
        scope,
        (if shown.cget() {
            view! {
                scope,
                div(class=format!("TraceTree {}", trace.kind.as_str())) {
                    TraceNode {
                        trace_id: props.trace_id,
                        attention,
                    }
                    (associated_trace_trees)
                    div {
                        Indexed {
                            iterable: subtrace_ids,
                            view: |scope, trace_id| view! {
                                scope,
                                TraceTree {
                                    trace_id,
                                }
                            },
                        }
                    }
                }
            }
        } else {
            view! {scope, }
        })
    }
}
