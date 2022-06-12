mod line_start;

use super::*;
use line_start::*;

#[derive(Prop)]
pub struct TraceLineProps<'a> {
    data: TraceLineData,
    trace_kind: TraceKind,
    has_subtraces: &'a ReadSignal<bool>,
    expanded: &'a ReadSignal<bool>,
}

#[component]
pub fn TraceLine<'a, G: Html>(scope: Scope<'a>, props: TraceLineProps<'a>) -> View<G> {
    let trace_tokens = View::new_fragment(
        props
            .data
            .tokens
            .iter()
            .map(|token_data| {
                view! { scope,
                    TraceToken {
                        data: token_data.clone(),
                    }
                }
            })
            .collect(),
    );
    view! {
        scope,
        p (class="TraceLine") {
            TraceLineStart {
                idx: props.data.idx,
                has_subtraces: props.has_subtraces,
                expanded: props.expanded,
                trace_kind: props.trace_kind,
            }
            (trace_tokens)
        }
    }
}

// <p class:unreachable>
//     <span class="indent" style="padding-left: {line.indent * 9.5}px" />
//     {#if line.idx === 0}
//         <span
//             class={`GroupStart ${trace.kind}`}
//             class:has_subtraces
//             class:expanded
//             on:mousedown={on_group_start_click}
//         >
//             {#if group_start_kind == "vscode"}
//                 <svg
//                     stroke="currentColor"
//                     fill="currentColor"
//                     stroke-width="0"
//                     viewBox="0 0 16 16"
//                     height="1.7em"
//                     width="2em"
//                     xmlns="http://www.w3.org/2000/svg"
//                 >
//                     <path
//                         fill-rule="evenodd"
//                         clip-rule="evenodd"
//                         d="M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619 4.357-4.357z"
//                     />
//                 </svg>
//             {:else}
//                 {group_start_kind}
//             {/if}
//         </span>
//     {:else}
//         <span class="GroupStart" />
//     {/if}
//     {#each line.tokens as token}
//         <Token {token} within_active_node={active} />
//     {/each}
//     {#each extra_tokens as token}
//         <Token {token} within_active_node={active} />
//     {/each}
// </p>
