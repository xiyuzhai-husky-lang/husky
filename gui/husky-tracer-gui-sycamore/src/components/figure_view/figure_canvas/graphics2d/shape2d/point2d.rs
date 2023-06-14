use super::*;

#[derive(Prop)]
pub struct Point2dProps<'a> {
    point: &'a Point2dData,
}

#[component]
pub fn Point2d<'a, G: Html>(visibility: Scope<'a>, props: Point2dProps<'a>) -> View<G> {
    view! {
        visibility,
        circle (
            cx={props.point.x},
            cy={props.point.y},
            r=0.1,
            fill="red"
        )
    }
}
