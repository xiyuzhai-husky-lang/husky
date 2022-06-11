use crate::*;

#[derive(Prop)]
pub struct MyProps<'a> {
    value: &'a ReadSignal<i32>,
}

#[component]
pub fn HSplitPanel<'a, G: Html>(cx: Scope<'a>, props: MyProps<'a>) -> View<G> {
    view! {
        cx,
        div(class="my-component") {
            "Value: " (props.value.get())
        }
    }
}
