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
pub fn TraceView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let context = use_dev_context(scope);
    let root_trace_ids_signal = &context.root_trace_ids_signal();
    view! {
        scope,
        div(class="TraceView disable-select") {
            Indexed {
                iterable: root_trace_ids_signal,
                view: |scope, trace_id| view! {
                    scope,
                    TraceTree {
                        trace_id
                    }
                },
            }
        }
    }
}
