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
    let ctx = use_dev_context(scope);
    let dimension = memo!(scope, || PixelDimension {
        width: props.width.cget(),
        height: props.height.cget()
    });
    view! {
        scope,
        VSplitPanel {
            dimension
        }
    }
}
