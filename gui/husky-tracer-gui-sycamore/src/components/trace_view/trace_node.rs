use super::*;

#[derive(Prop)]
pub struct TraceNodeProps<'a> {
    trace_id: TraceId,
    has_subtraces: &'a ReadSignal<bool>,
    expansion: &'a ReadSignal<bool>,
}

#[component]
pub fn TraceNode<'a, G: Html>(scope: Scope<'a>, props: TraceNodeProps<'a>) -> View<G> {
    let tracer_context = use_context::<TracerContext>(scope);
    let tree_context = &tracer_context.tree_context;
    let shown = tree_context.shown_signal(props.trace_id);
    let expansion = tree_context.expanded_signal(props.trace_id);
    let focus = tracer_context.focus_context.focus_signal.clone();
    let trace = tree_context.trace(props.trace_id);
    let trace_kind = trace.kind;
    let has_stalk = create_memo(scope, { move || focus.get().has_stalk(trace_kind) });
    let trace_lines = View::new_fragment(
        trace
            .lines
            .iter()
            .map(|line_data| {
                view! { scope,
                    TraceLine {
                        data: line_data.clone(),
                        trace_kind,
                        has_subtraces: props.has_subtraces,
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

// <div class="inner" class:active>
// {#each trace.lines as line}
//     <Line
//         {trace}
//         {on_group_start_click}
//         {expanded}
//         {has_subtraces}
//         {active}
//         {line}
//         {extra_tokens}
//     />
// {/each}
// </div>
