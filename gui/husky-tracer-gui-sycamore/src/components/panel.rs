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
    let left_style = "width: 800px";
    let right_style = "width: 800px";
    let inner_width = create_signal(scope, 0);
    view! {
        scope,
        div(class="HuskyTracerHSplitPanel") {
            div(class="HuskyTracerHSplitPanelLeft", style=left_style) {
                TraceView {}
            }
            div(class="HuskyTracerHSplitPanelRight", style=right_style) {
                "Value: " (props.value.get())
            }
        }
    }
}
