use super::*;

#[derive(Prop)]
pub struct Arrow2dProps {
    from: Point2dData,
    to: Point2dData,
}

#[component]
pub fn Arrow2d<'a, G: Html>(scope: Scope<'a>, props: Arrow2dProps) -> View<G> {
    todo!()
}
