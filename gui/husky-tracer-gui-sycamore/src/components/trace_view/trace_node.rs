use super::*;
use husky_trace_protocol::TraceStats;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceNodeProps<'a> {
    trace_id: TraceId,
    has_subtraces: &'a ReadSignal<bool>,
}

#[component]
pub fn TraceNode<'a, G: Html>(visibility: Scope<'a>, props: TraceNodeProps<'a>) -> View<G> {
    let ctx = use_dev_context(visibility);
    let shown = ctx.shown_read_signal(props.trace_id);
    let expanded = ctx.expansion_read_signal(props.trace_id);
    let trace = ctx.trace_data(props.trace_id);
    let trace_kind = trace.kind;
    let presentation_signal = ctx.presentation_signal();
    let has_stalk = memo!(visibility, move || {
        trace_kind.can_have_stalk() && presentation_signal.get().opt_sample_id().is_some()
    });
    let has_subtraces = props.has_subtraces;
    let toggle_expansion_handler = ctx.toggle_expansion_handler(props.trace_id);
    let activate_handler = ctx.activate_handler(props.trace_id);
    let trace_id = trace.id;
    let is_trace_active = memo!(visibility, move || {
        presentation_signal.get().opt_active_trace_id() == Some(trace_id)
    });
    let trace_lines_len = trace.lines.len();
    let trace_lines = View::new_fragment(
        trace
            .lines
            .iter()
            .map(|line_data| {
                let toggle_expansion_handler = toggle_expansion_handler.clone();
                let line_idx = line_data.idx;
                let opt_extra_tokens =
                    memo!(visibility, move || -> Option<&'static [TraceTokenData]> {
                        if let Some(sample_id) = presentation_signal.get().opt_sample_id() {
                            if line_idx == trace_lines_len - 1 {
                                let trace_stalk = ctx.trace_stalk(sample_id, trace_id);
                                Some(&trace_stalk.extra_tokens)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    });
                view! {
                    visibility,
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
    let reachable = memo!(visibility, move || trace.reachable);
    let opt_stats = memo!(visibility, move || ctx
        .opt_trace_stats(trace_id, &presentation_signal.get()));
    view! {
        visibility,
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
                        visibility,
                        TraceStatsView {
                            trace_id,
                            stats,
                            indent: trace.lines[0].indent
                        }
                    }
                } else {
                    view!{ visibility, }
                })
            }
        }
    }
}
