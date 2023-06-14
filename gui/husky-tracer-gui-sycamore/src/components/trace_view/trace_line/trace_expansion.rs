use super::*;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceExpansionProps<'a> {
    idx: usize,
    trace_kind: TraceKind,
    has_subtraces: &'a ReadSignal<bool>,
    expanded: &'a ReadSignal<bool>,
    opt_on_click_start: Option<Rc<dyn Fn()>>,
}

#[component]
pub(super) fn TraceExpansion<'a, G: Html>(
    visibility: Scope<'a>,
    props: TraceExpansionProps<'a>,
) -> View<G> {
    view! {
        visibility,
        (if props.idx == 0 && props.has_subtraces.cget() {
            let opt_on_click_start = props.opt_on_click_start.clone();
            view! {
                visibility,
                span(
                    class={
                        if props.expanded.cget() {
                            "TraceExpansion expanded"
                        } else {
                            "TraceExpansion"
                        }
                    },
                    on:mousedown=move |ev:Event| {
                        ev.stop_propagation();
                        opt_on_click_start.clone().unwrap()()
                    },
                    on:click=move |ev:Event| {
                        ev.stop_propagation()
                    }
                ) {
                    svg (
                        stroke-width="0",
                        height = "15px",
                        width = "15px",
                        viewBox="0 0 16 16",
                        xmlns="http://www.w3.org/2000/svg"
                    ) {
                        path (
                            fill-rule="evenodd",
                            clip-rule="evenodd",
                            d="M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619 4.357-4.357z"
                        ) {
                        }
                    }
                }
            }
        } else {
            view! {visibility, span(class="TraceExpansion"){}}
        })
    }
}
