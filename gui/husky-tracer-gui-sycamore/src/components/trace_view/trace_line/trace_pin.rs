use super::*;
use web_sys::{Event, MouseEvent};

#[derive(Prop)]
pub struct TracePinProps {
    trace_id: TraceId,
    line_idx: usize,
}

#[component]
pub(super) fn TracePin<'a, G: Html>(visibility: Scope<'a>, props: TracePinProps) -> View<G> {
    let ctx = use_dev_context(visibility);
    let trace_id = props.trace_id;
    let presentation_signal = ctx.presentation_signal();
    let pinned = memo!(visibility, move || presentation_signal
        .get()
        .is_pinned(trace_id));
    if props.line_idx == 0 {
        view! {
            visibility,
            span(
                class={
                    if pinned.cget() {
                        "TracePin pinned"
                    } else {
                        "TracePin ignored"
                    }
                },
                on:mousedown=move |ev:Event|{
                    ev.stop_propagation() ;
                    ctx.toggle_pin_handler(trace_id)()
                },
                on:click=move |ev:Event| {
                    ev.stop_propagation()
                }
            ) {
                svg (
                    stroke="currentColor",
                    fill="currentColor",
                    stroke-width="0",
                    viewBox="0 0 24 24",
                    height="1em", width="1em",
                    xmlns="http://www.w3.org/2000/svg"
                ){
                    path (d="M12,22l1-2v-3h5c0.553,0,1-0.447,1-1v-1.586c0-0.526-0.214-1.042-0.586-1.414L17,11.586V8c0.553,0,1-0.447,1-1V4 c0-1.103-0.897-2-2-2H8C6.897,2,6,2.897,6,4v3c0,0.553,0.448,1,1,1v3.586L5.586,13C5.213,13.372,5,13.888,5,14.414V16 c0,0.553,0.448,1,1,1h5v3L12,22z M8,4h8v2H8V4z M7,14.414l1.707-1.707C8.895,12.52,9,12.266,9,12V8h6v4 c0,0.266,0.105,0.52,0.293,0.707L17,14.414V15H7V14.414z")
                }
            }
        }
    } else {
        view! {visibility, span(class="TraceExpansion"){}}
    }
}
