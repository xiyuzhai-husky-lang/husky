use super::*;

#[derive(Prop)]
pub struct TraceNodeProps<'a> {
    trace_id: TraceId,
    expansion: &'a ReadSignal<bool>,
    focus: &'a ReadSignal<Focus>,
}

#[component]
pub fn TraceNode<'a, G: Html>(scope: Scope<'a>, props: TraceNodeProps<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let tree_context = &tracer_context.tree_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let expansion = tree_context.expanded_signal(props.trace_id);
    let trace = tree_context.trace(props.trace_id);
    let trace_kind = trace.kind;
    let focus = props.focus;
    let has_stalk = create_memo(scope, { move || focus.get().has_stalk(trace_kind) });
    let can_have_subtraces = trace.can_have_subtraces;
    let has_subtraces = create_memo(scope, move || {
        tell_has_subtraces(trace_kind, can_have_subtraces, &focus.get())
    });
    let trace_lines = View::new_fragment(
        trace
            .lines
            .iter()
            .map(|line_data| {
                view! { scope,
                    TraceLine {
                        data: line_data.clone(),
                        trace_kind,
                        has_subtraces,
                        expanded: props.expansion
                    }
                }
            })
            .collect(),
    );
    view! {
        scope,
        div(class="TraceNode") {
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
    // switch (trace.kind) {
    //     case "Main":
    //     case "FeatureBranch":
    //     case "LoopFrame":
    //         return readable(true);
    //     case "CallHead":
    //     case "FeatureCallInput":
    //     case "FeatureStmt":
    //         return readable(false);
    //     case "FuncStmt":
    //     case "EagerExpr":
    //     case "ProcStmt":
    //     case "ProcBranch":
    //         return readable(trace.has_subtraces);
    //     case "FeatureExpr":
    //         let focus_store = state.focus_state.focus_store;
    //         return derived(
    //             focus_store,
    //             ($focus_store) =>
    //                 $focus_store.opt_input_id !== null &&
    //                 trace.has_subtraces
    //         );
    // }
}
