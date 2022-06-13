mod hsplit_panel;
mod vsplit_panel;

use super::*;
use vsplit_panel::*;

#[derive(Prop, Clone)]
pub struct Layout<'a> {
    width: &'a ReadSignal<f64>,
    height: &'a ReadSignal<f64>,
}

#[component]
pub fn Layout<'a, G: Html>(scope: Scope<'a>, props: Layout<'a>) -> View<G> {
    view! {
        scope,
        VSplitPanel {
            width: props.width,
            height: props.height,
        }
    }
}
