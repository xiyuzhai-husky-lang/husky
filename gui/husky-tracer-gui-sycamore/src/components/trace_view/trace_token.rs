use super::*;

#[derive(Prop)]
pub struct TraceTokenProps<'a> {
    data: &'a TraceTokenData,
}

#[component]
pub fn TraceToken<'a, G: Html>(scope: Scope<'a>, props: TraceTokenProps<'a>) -> View<G> {
    let text = &props.data.value;
    let spaces_before_style = spaces_style(count_spaces_before(text));
    let spaces_after_style = spaces_style(count_spaces_after(text));
    let token_kind = props.data.kind;
    view! {
        scope,
        span (style=spaces_before_style)
        code(class=format!("TraceToken {}", token_kind)) {
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
    let width = n as f64 * 9.5;
    format!("width: {width}px")
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
