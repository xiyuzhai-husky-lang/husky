use super::*;
use husky_vm_primitive_value::PrimitiveValueData;

#[derive(Prop)]
pub struct PrimitiveValueCanvasProps {
    value: PrimitiveValueData,
}

#[component]
pub fn PrimitiveValueCanvas<'a, G: Html>(
    visibility: Scope<'a>,
    props: PrimitiveValueCanvasProps,
) -> View<G> {
    view! {visibility, }
}
