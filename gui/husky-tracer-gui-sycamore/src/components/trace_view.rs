mod trace_line;
mod trace_node;
mod trace_token;
mod trace_tree;

use super::*;
use trace_line::*;
use trace_node::*;
use trace_token::*;
use trace_tree::*;

#[component]
pub fn TraceView<'a, G: Html>(scope: Scope<'a>) -> View<G> {
    let context = use_debugger_context(scope);
    let root_trace_ids = &context.trace_context.root_trace_ids;
    view! {
        scope,
        div(class="TraceView disable-select") {
            Indexed {
                iterable: root_trace_ids,
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
