mod trace_expansion;
mod trace_pin;
mod trace_token;

use super::*;
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
pub fn TraceLine<'a, G: Html>(visibility: Scope<'a>, props: TraceLineProps<'a>) -> View<G> {
    let trace_tokens = View::new_fragment(
        props
            .data
            .tokens
            .iter()
            .map(|token_data| {
                view! { visibility,
                    TraceToken {
                        is_trace_active: props.is_trace_active,
                        data: token_data,
                    }
                }
            })
            .collect(),
    );
    let extra_tokens = memo!(visibility, move || View::new_fragment(
        if let Some(extra_tokens) = props.opt_extra_tokens.cget() {
            extra_tokens
                .iter()
                .map(|token_data| {
                    view! { visibility,
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
        visibility,
        div(class="TraceLine"){
            p (class="TraceLineLeft") {
                span (class="indent", style=format!("padding-left: {}ch", indent))
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
                TracePin {
                    line_idx,
                    trace_id,
                }
            }
        }
    }
}
