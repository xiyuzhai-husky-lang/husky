use super::*;

#[derive(Prop)]
pub struct TraceLineStartProps<'a> {
    idx: usize,
    trace_kind: TraceKind,
    has_subtraces: &'a ReadSignal<bool>,
    expanded: &'a ReadSignal<bool>,
}

#[component]
pub(super) fn TraceLineStart<'a, G: Html>(
    scope: Scope<'a>,
    props: TraceLineStartProps<'a>,
) -> View<G> {
    if props.idx == 0 && props.has_subtraces.get_cloned() {
        view! {scope, span(class="TraceLineStart"){
            svg (
                height = "1.7em",
                width = "2em",
                view_box="0 0 16 16",
                xmlns="http://www.w3.org/2000/svg"
            ) {
                path (
                    fill-rule="evenodd",
                    clip-rule="evenodd",
                    d="M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619 4.357-4.357z"
                ) {
                }
            }
        }}
    } else {
        view! {scope, span(class="TraceLineStart"){}}
    }
}
