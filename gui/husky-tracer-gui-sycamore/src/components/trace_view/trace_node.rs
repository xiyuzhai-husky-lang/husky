use web_sys::Event;

use super::*;

#[derive(Prop)]
pub struct TraceNodeProps<'a> {
    trace_id: TraceId,
    attention: &'a ReadSignal<Attention>,
}

#[component]
pub fn TraceNode<'a, G: Html>(scope: Scope<'a>, props: TraceNodeProps<'a>) -> View<G> {
    let debuggerer_context = use_context::<DebuggerContext>(scope);
    let trace_context = &debuggerer_context.trace_context;
    let shown = trace_context.shown_signal(props.trace_id);
    let expansion = trace_context.expanded_signal(props.trace_id);
    let expanded = memo!(scope, move || expansion.cget(), expansion);
    let trace = trace_context.trace(props.trace_id);
    let trace_kind = trace.kind;
    let attention = props.attention;
    let has_stalk = memo!(scope, move || attention.get().has_stalk(trace_kind));
    let can_have_subtraces = trace.can_have_subtraces;
    let has_subtraces = memo!(scope, move || {
        tell_has_subtraces(trace_kind, can_have_subtraces, &attention.get())
    });
    let toggle_expansion_handler = debuggerer_context.toggle_expansion_handler(props.trace_id);
    let activate_handler = debuggerer_context.activate_handler(props.trace_id);
    let opt_active_trace_id = &trace_context.opt_active_trace_id;
    let trace_id = trace.id;
    let active = memo!(scope, move || opt_active_trace_id.cget() == Some(trace_id));
    let trace_lines_len = trace.lines.len();
    let trace_lines = View::new_fragment(
        trace
            .lines
            .iter()
            .map(|line_data| {
                let toggle_expansion_handler = toggle_expansion_handler.clone();
                let line_idx = line_data.idx;
                let opt_extra_tokens = memo!(scope, move || {
                    if let Some(sample_idx) = attention.get().opt_sample_idx() {
                        if line_idx == trace_lines_len - 1 {
                            let trace_stalk = trace_context.trace_stalk(sample_idx, trace_id);
                            trace_stalk.opt_extra_tokens.clone()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });
                view! { scope,
                    TraceLine {
                        data: line_data.clone(),
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
    view! {
        scope,
        div(
            class="TraceNode",
            on:mousedown=activate_handler
        ) {
            div(
                class={
                    if active.cget() {
                        "TraceNodeInternal active"
                    } else {
                        "TraceNodeInternal"
                    }
                },
            ) {
                (trace_lines)
            }
        }
    }
}

fn tell_has_subtraces(
    trace_kind: TraceKind,
    can_have_subtraces: bool,
    attention: &Attention,
) -> bool {
    match trace_kind {
        TraceKind::Main | TraceKind::FeatureBranch | TraceKind::LoopFrame => true,
        TraceKind::CallHead | TraceKind::FeatureCallInput | TraceKind::FeatureStmt => false,
        TraceKind::FuncStmt
        | TraceKind::EagerExpr
        | TraceKind::ProcStmt
        | TraceKind::ProcBranch => can_have_subtraces,
        TraceKind::FeatureExpr => todo!(),
    }
}
