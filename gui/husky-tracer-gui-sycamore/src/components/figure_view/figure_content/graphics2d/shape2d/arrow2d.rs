use super::*;

#[derive(Prop)]
pub struct Arrow2dProps<'a> {
    from: &'a Point2dData,
    to: &'a Point2dData,
}

#[component]
pub fn Arrow2d<'a, G: Html>(scope: Scope<'a>, props: Arrow2dProps<'a>) -> View<G> {
    todo!()
}
