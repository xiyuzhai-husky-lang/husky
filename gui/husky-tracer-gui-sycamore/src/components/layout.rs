mod hsplit_panel;
mod vsplit_panel;

use super::*;
use vsplit_panel::*;

#[derive(Prop, Clone)]
pub struct LayoutProps<'a> {
    width: &'a ReadSignal<u32>,
    height: &'a ReadSignal<u32>,
}

#[component]
pub fn Layout<'a, G: Html>(scope: Scope<'a>, props: LayoutProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let root_trace_ids = &context.tree_context.root_trace_ids;
    view! {
        scope,
        VSplitPanel {
            width: props.width,
            height: props.height,
        }
    }
}
