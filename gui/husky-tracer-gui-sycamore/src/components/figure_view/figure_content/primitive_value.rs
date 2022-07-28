use super::*;

#[derive(Prop)]
pub struct PrimitiveValueCanvasProps {
    value: husky_trace_protocol::PrimitiveValueData,
}

#[component]
pub fn PrimitiveValueCanvas<'a, G: Html>(
    scope: Scope<'a>,
    props: PrimitiveValueCanvasProps,
) -> View<G> {
    view! {scope, }
}
