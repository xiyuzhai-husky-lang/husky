use super::*;

#[derive(Prop)]
pub struct TraceNodeProps<'a> {
    trace_id: TraceId,
    focus: &'a ReadSignal<Focus>,
}

#[component]
pub fn TraceNode<'a, G: Html>(scope: Scope<'a>, props: TraceNodeProps<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let tree_context = &tracer_context.tree_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let expansion = tree_context.expanded_signal(props.trace_id);
    let expanded = memo!(scope, expansion.get_cloned(), expansion);
    let trace = tree_context.trace(props.trace_id);
    let trace_kind = trace.kind;
    let focus = props.focus;
    let has_stalk = memo!(scope, focus.get().has_stalk(trace_kind));
    let can_have_subtraces = trace.can_have_subtraces;
    let has_subtraces = create_memo(scope, move || {
        tell_has_subtraces(trace_kind, can_have_subtraces, &focus.get())
    });
    let toggle_expansion_handler = tracer_context.toggle_expansion_handler(props.trace_id);
    let activate_handler = tracer_context.activate_handler(props.trace_id);
    let opt_active_trace_id = &tree_context.opt_active_trace_id;
    let trace_id = trace.id;
    let active = memo!(scope, opt_active_trace_id.get_cloned() == Some(trace_id));
    let trace_lines = View::new_fragment(
        trace
            .lines
            .iter()
            .map(|line_data| {
                let toggle_expansion_handler = toggle_expansion_handler.clone();
                view! { scope,
                    TraceLine {
                        data: line_data.clone(),
                        trace_kind,
                        has_subtraces,
                        expanded,
                        toggle_expansion_handler
                    }
                }
            })
            .collect(),
    );
    view! {
        scope,
        div(
            class=if active.get_cloned() { "TraceNode active" } else { "TraceNode" },
            on:mousedown=activate_handler
        ) {
            (trace_lines)
        }
    }
}

fn tell_has_subtraces(trace_kind: TraceKind, can_have_subtraces: bool, focus: &Focus) -> bool {
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
