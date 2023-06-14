use super::*;
use web_sys::Event;
use web_sys::MouseEvent;

#[derive(Prop)]
pub struct TraceTokenProps<'a> {
    is_trace_active: &'a ReadSignal<bool>,
    data: &'a TraceTokenData,
}

#[component]
pub fn TraceToken<'a, G: Html>(visibility: Scope<'a>, props: TraceTokenProps<'a>) -> View<G> {
    let text = &props.data.value;
    let spaces_before_style = spaces_style(count_spaces_before(text));
    let spaces_after_style = spaces_style(count_spaces_after(text));
    let token_kind = props.data.kind;
    let context = use_dev_context(visibility);
    let shown = memo!(visibility, move || {
        if let Some(associated_trace_id) = props.data.opt_associated_trace_id {
            context.shown_read_signal(associated_trace_id).cget()
        } else {
            false
        }
    });
    view! {
        visibility,
        span (style=spaces_before_style)
        code(
            class=format!("TraceToken {} {}", token_kind,
                if shown.cget() {
                    "associated_trace_shown"
                } else {
                    ""
                }
            ),
            on:mousedown=move |_ev:Event|{
                if props.is_trace_active.cget() {
                    if let Some(associated_trace_id) = props.data.opt_associated_trace_id {
                        use_dev_context(visibility).toggle_shown_handler(associated_trace_id)()
                    }
                }
            }
        ) {
            (props.data.value)
        }
        span (style=spaces_after_style)
    }
}

fn count_spaces_before(text: &str) -> usize {
    for (i, c) in text.chars().enumerate() {
        if (c != ' ') {
            return i;
        }
    }
    text.chars().count()
}
fn count_spaces_after(text: &str) -> usize {
    for (i, c) in text.chars().rev().enumerate() {
        if c != ' ' {
            return i;
        }
    }
    return 0;
}
fn spaces_style(n: usize) -> String {
    format!("width: {n}ch")
}

// <span style={spacesBeforeStyles} />
// <code
//     class={token.kind}
//     class:associated
//     class:associated_trace_shown
//     on:click={handleClick}
// >
//     {token.value}
// </code>
// <span style={spacesAfterStyles} />
