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
    let focus = tracer_context.focus_context.focus.clone();
    let focus = memo!(scope, move || focus.cget());
    let associated_trace_trees = View::new_fragment(
        tree_context
            .trace(props.trace_id)
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
                    .subtrace_ids(&focus.get(), props.trace_id)
                    .to_vec()
            } else {
                vec![]
            }
        }
    });
    if shown.cget() {
        view! {
            scope,
            div(class="TraceTree") {
                TraceNode {
                    trace_id: props.trace_id,
                    focus,
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
    }
}
