use crate::*;

#[derive(Prop)]
pub struct MyProps<'a> {
    value: &'a ReadSignal<i32>,
}

#[component]
pub fn HSplitPanel<'a, G: Html>(scope: Scope<'a>, props: MyProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    log::info!("{:?}", context.borrow().tree_context.root_trace_ids);
    view! {
        scope,
        div(class="my-component") {
            "Value: " (props.value.get())
        }
    }
}
