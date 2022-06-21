use super::*;

#[derive(Prop)]
pub struct GenericI32Props {
    points: Vec<i32>,
}

#[component]
pub fn GenericI32<'a, G: Html>(scope: Scope<'a>, props: GenericI32Props) -> View<G> {
    let max = props.points.iter().max().unwrap();
    todo!()
}
