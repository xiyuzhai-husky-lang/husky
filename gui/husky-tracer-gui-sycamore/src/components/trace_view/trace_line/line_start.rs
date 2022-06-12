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
                style = "height: 1.7em; width: 2em",
                view_box="0 0 16 16"
            ) {

            }
        }}
    } else {
        view! {scope, span(class="TraceLineStart"){}}
    }
}
