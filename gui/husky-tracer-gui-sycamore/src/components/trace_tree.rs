use crate::*;

pub struct TraceTreeProps {}

#[component]
pub fn TraceTree<'a, G: Html>(scope: Scope<'a>, props: TraceTreeProps) -> View<G> {
    view! {
        scope,
        div(class="TraceTree") {
        }
    }
}
