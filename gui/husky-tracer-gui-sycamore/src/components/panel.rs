use super::*;

#[derive(Prop)]
pub struct MyProps<'a> {
    value: &'a ReadSignal<i32>,
}

#[component]
pub fn HSplitPanel<'a, G: Html>(scope: Scope<'a>, props: MyProps<'a>) -> View<G> {
    let context = use_context::<TracerContext>(scope);
    let root_trace_ids = &context.tree_context.root_trace_ids;
    create_effect(scope, move || {
        log::info!("root traces {:?}", root_trace_ids)
    });
    view! {
        scope,
        div(class="my-component") {
            "Value: " (props.value.get())
        }
        TraceView {}
    }
}
