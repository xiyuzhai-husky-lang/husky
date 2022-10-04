use super::*;

#[derive(Prop)]
pub struct TraceTreeProps {
    trace_id: TraceId,
}

#[component]
pub fn TraceTree<'a, G: Html>(scope: Scope<'a>, props: TraceTreeProps) -> View<G> {
    let ctx = use_dev_context(scope);
    let tree_context = &ctx.trace_context;
    let shown = tree_context.shown_read_signal(props.trace_id);
    let opt_sample_id = ctx.opt_sample_id_signal();
    let trace = tree_context.trace_data(props.trace_id);
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
    let has_subtraces = memo!(scope, {
        move || trace.has_subtraces(opt_sample_id.cget().is_some())
    });
    let subtrace_ids = memo!(scope, {
        let expansion = tree_context.expansion_read_signal(props.trace_id);
        move || {
            if expansion.cget() {
                tree_context
                    .subtrace_ids(props.trace_id, opt_sample_id.cget())
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
                        has_subtraces
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
