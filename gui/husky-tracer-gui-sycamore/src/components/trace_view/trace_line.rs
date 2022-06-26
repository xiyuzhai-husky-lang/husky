mod trace_arrival;
mod trace_expansion;
mod trace_pin;
mod trace_token;

use super::*;
use trace_arrival::*;
use trace_expansion::*;
use trace_pin::*;
use trace_token::*;

#[derive(Prop)]
pub struct TraceLineProps<'a> {
    data: &'a TraceLineData,
    is_trace_active: &'a ReadSignal<bool>,
    trace_id: TraceId,
    trace_kind: TraceKind,
    has_subtraces: &'a ReadSignal<bool>,
    expanded: &'a ReadSignal<bool>,
    toggle_expansion_handler: Rc<dyn Fn()>,
    opt_extra_tokens: &'a ReadSignal<Option<&'static [TraceTokenData]>>,
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
                        is_trace_active: props.is_trace_active,
                        data: token_data,
                    }
                }
            })
            .collect(),
    );
    let extra_tokens = memo!(scope, move || View::new_fragment(
        if let Some(extra_tokens) = props.opt_extra_tokens.cget() {
            extra_tokens
                .iter()
                .map(|token_data| {
                    view! { scope,
                        TraceToken {
                            is_trace_active: props.is_trace_active,
                            data: token_data,
                        }
                    }
                })
                .collect()
        } else {
            vec![]
        },
    ));
    let indent = props.data.indent;
    let line_idx = props.data.idx;
    let trace_id = props.trace_id;
    view! {
        scope,
        div(class="TraceLine"){
            p (class="TraceLineLeft") {
                span (class="indent", style=format!("padding-left: {}px", indent as f64 * 9.5))
                TraceExpansion {
                    idx: props.data.idx,
                    has_subtraces: props.has_subtraces,
                    expanded: props.expanded,
                    trace_kind: props.trace_kind,
                    opt_on_click_start:{
                        if  props.data.idx == 0 {
                            Some(props.toggle_expansion_handler.clone())
                        } else {
                            None
                        }
                    },
                }
                (trace_tokens)
                (extra_tokens.cget())
            }
            div(class="TraceLineRight") {
                TraceArrival {
                    line_idx,
                    trace_id,
                }
                TracePin {
                    line_idx,
                    trace_id,
                }
            }
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
