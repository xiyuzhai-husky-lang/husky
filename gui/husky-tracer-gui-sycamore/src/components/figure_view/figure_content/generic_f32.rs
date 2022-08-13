use super::*;

#[derive(Prop)]
pub struct GenericF32Props<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: &'a [(PartitionDefnData, Vec<(SampleId, f32)>)],
}

#[component]
pub fn GenericF32<'a, G: Html>(scope: Scope<'a>, props: GenericF32Props<'a>) -> View<G> {
    todo!()
}
