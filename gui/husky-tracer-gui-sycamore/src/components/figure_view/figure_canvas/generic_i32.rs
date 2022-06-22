use super::*;

#[derive(Prop)]
pub struct GenericI32Props {
    points: Vec<(SampleId, i32)>,
}

#[component]
pub fn GenericI32<'a, G: Html>(scope: Scope<'a>, props: GenericI32Props) -> View<G> {
    // let max = props.points.iter().max().unwrap();
    todo!()
}
