use super::*;

#[derive(Prop)]
pub struct TraceTokenProps {
    data: TraceTokenData,
}

#[component]
pub fn TraceToken<'a, G: Html>(scope: Scope<'a>, props: TraceTokenProps) -> View<G> {
    view! {
        scope,
        code(class=props.data.kind) {
            (props.data.value)
        }
    }
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
