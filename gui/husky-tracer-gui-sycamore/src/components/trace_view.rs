mod trace_line;
mod trace_node;
mod trace_stats;
mod trace_tree;

use super::*;
use trace_line::*;
use trace_node::*;
use trace_stats::*;
use trace_tree::*;

#[component]
pub fn TraceView<'a, G: Html>(visibility: Scope<'a>) -> View<G> {
    let context = use_dev_context(visibility);
    let root_trace_ids_signal = &context.root_trace_ids_signal();
    view! {
        visibility,
        div(class="TraceView disable-select") {
            Indexed {
                iterable: root_trace_ids_signal,
                view: |visibility, trace_id| view! {
                    visibility,
                    TraceTree {
                        trace_id
                    }
                },
            }
        }
    }
}
