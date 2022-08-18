use super::*;
use web_sys::Event;

#[derive(Prop)]
pub struct TraceArrivalProps {
    trace_id: TraceId,
    line_idx: usize,
}

#[component]
#[component]
pub(super) fn TraceArrival<'a, G: Html>(scope: Scope<'a>, props: TraceArrivalProps) -> View<G> {
    let ctx = use_debugger_context(scope);
    let trace_context = &ctx.trace_context;
    let trace_id = props.trace_id;
    let arrival = memo!(scope, move || ctx
        .restriction_context
        .restriction
        .get()
        .arrival(trace_id));
    let trace_id = props.trace_id;
    if props.line_idx == 0 {
        view! {
            scope,
            (if arrival.cget() {
                view! {
                    scope,
                    div (class = "TraceArrivalRefinedControl") {
                        svg (
                            stroke="currentColor",
                            fill="currentColor",
                            stroke-width="0",
                            viewBox="0 0 16 16",
                            height="0.8em",
                            width="0.8em",
                            xmlns="http://www.w3.org/2000/svg"
                        ) {
                            path (
                                fill-rule="evenodd",
                                clip-rule="evenodd",
                                d="M14.431 3.323l-8.47 10-.79-.036-3.35-4.77.818-.574 2.978 4.24 8.051-9.506.764.646z"
                            )
                        }
                        svg (
                            stroke="currentColor",
                            fill="currentColor",
                            stroke-width="0",
                            viewBox="0 0 16 16",
                            height="0.8em",
                            width="0.8em",
                            xmlns="http://www.w3.org/2000/svg"
                        ) {
                            path (
                                fill-rule="evenodd",
                                clip-rule="evenodd",
                                d="M7.116 8l-4.558 4.558.884.884L8 8.884l4.558 4.558.884-.884L8.884 8l4.558-4.558-.884-.884L8 7.116 3.442 2.558l-.884.884L7.116 8z"
                            )
                        }
                        svg (
                            stroke="currentColor",
                            fill="currentColor",
                            stroke-width="0",
                            viewBox="0 0 24 24",
                            height="0.8em",
                            width="0.8em",
                            xmlns="http://www.w3.org/2000/svg"
                        ) {
                            path (
                                stroke-linecap="round",
                                stroke-linejoin="round",
                                stroke-width="2",
                                d="M13 10V3L4 14h7v7l9-11h-7z"
                            )
                        }
                    }
                }
            } else {
                view! { scope, }
            })
            span(
                class={
                    if arrival.cget() {
                        "TraceArrival arrival"
                    } else {
                        "TraceArrival ignored"
                    }
                },
                on:mousedown=move |ev:Event|{
                    ev.stop_propagation();
                    ctx.toggle_arrival_handler(trace_id)()
                }
            ) {
                svg (
                    stroke="currentColor",
                    fill="currentColor",
                    stroke-width="0",
                    viewBox="0 0 24 24",
                    height="0.9em", width="0.9em",
                    xmlns="http://www.w3.org/2000/svg"
                ){
                    path (d="M12,17l1-2V9.858c1.721-0.447,3-2,3-3.858c0-2.206-1.794-4-4-4S8,3.794,8,6c0,1.858,1.279,3.411,3,3.858V15L12,17z M10,6 c0-1.103,0.897-2,2-2s2,0.897,2,2s-0.897,2-2,2S10,7.103,10,6z")
                    path (d="M16.267,10.563l-0.533,1.928C18.325,13.207,20,14.584,20,16c0,1.892-3.285,4-8,4s-8-2.108-8-4 c0-1.416,1.675-2.793,4.267-3.51l-0.533-1.928C4.197,11.54,2,13.623,2,16c0,3.364,4.393,6,10,6s10-2.636,10-6 C22,13.623,19.803,11.54,16.267,10.563z")
                }
            }
        }
    } else {
        view! {scope, span(class="TraceExpansion"){}}
    }
}
