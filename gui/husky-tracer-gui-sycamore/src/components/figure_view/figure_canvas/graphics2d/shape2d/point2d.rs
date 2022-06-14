use super::*;

#[derive(Prop)]
pub struct Point2dProps {
    point: Point2dData,
}

#[component]
pub fn Point2d<'a, G: Html>(scope: Scope<'a>, props: Point2dProps) -> View<G> {
    view! {
        scope,
        circle (
            cx={props.point.x},
            cy={props.point.y}
        )
    }
}
