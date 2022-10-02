use super::*;
use husky_trace_protocol::TraceStats;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceNodeProps<'a> {
    trace_id: TraceId,
    has_subtraces: &'a ReadSignal<bool>,
}

#[component]
pub fn TraceNode<'a, G: Html>(scope: Scope<'a>, props: TraceNodeProps<'a>) -> View<G> {
    let dev_context = use_dev_context(scope);
    let trace_context = &dev_context.trace_context;
    let shown = trace_context.shown_read_signal(props.trace_id);
    let expanded = trace_context.expansion_read_signal(props.trace_id);
    let trace = trace_context.trace_data(props.trace_id);
    let trace_kind = trace.kind;
    let opt_sample_id = dev_context.restriction_context.opt_sample_id;
    let has_stalk = memo!(scope, move || trace_kind.can_have_stalk()
        && opt_sample_id.cget().is_some());
    let has_subtraces = props.has_subtraces;
    let toggle_expansion_handler = dev_context.toggle_expansion_handler(props.trace_id);
    let activate_handler = dev_context.activate_handler(props.trace_id);
    let opt_active_trace_id = &trace_context.opt_active_trace_id;
    let trace_id = trace.id;
    let is_trace_active = memo!(scope, move || opt_active_trace_id.cget() == Some(trace_id));
    let trace_lines_len = trace.lines.len();
    let trace_lines = View::new_fragment(
        trace
            .lines
            .iter()
            .map(|line_data| {
                let toggle_expansion_handler = toggle_expansion_handler.clone();
                let line_idx = line_data.idx;
                let opt_extra_tokens =
                    memo!(scope, move || -> Option<&'static [TraceTokenData]> {
                        if let Some(sample_id) = opt_sample_id.cget() {
                            if line_idx == trace_lines_len - 1 {
                                let trace_stalk = trace_context.trace_stalk(sample_id, trace_id);
                                Some(&trace_stalk.extra_tokens)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    });
                view! {
                    scope,
                    TraceLine {
                        data: line_data,
                        is_trace_active,
                        trace_id,
                        trace_kind,
                        has_subtraces,
                        expanded,
                        toggle_expansion_handler,
                        opt_extra_tokens,
                    }
                }
            })
            .collect(),
    );
    let reachable = memo!(scope, move || trace.reachable);
    let restriction = dev_context.restriction_context.restriction;
    let opt_stats = memo!(scope, move || trace_context
        .opt_trace_stats(trace_id, &restriction.get()));
    view! {
        scope,
        div(
            class=format!("TraceNode {}", class!(*reachable)),
            on:mousedown=activate_handler
        ) {
            div(
                class={
                    if is_trace_active.cget() {
                        "TraceNodeInternal active"
                    } else {
                        "TraceNodeInternal"
                    }
                },
            ) {
                (trace_lines)
                (if let Some(ref stats) = *opt_stats.get() {
                    view!{
                        scope,
                        TraceStatsView {
                            stats,
                            indent: trace.lines[0].indent
                        }
                    }
                } else {
                    view!{ scope, }
                })
            }
        }
    }
}
