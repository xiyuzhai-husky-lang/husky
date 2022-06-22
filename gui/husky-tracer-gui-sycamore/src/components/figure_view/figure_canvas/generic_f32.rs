use super::*;

#[derive(Prop)]
pub struct GenericI32Props<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: Vec<(Rc<PartitionDefnData>, Vec<(SampleId, i32)>)>,
}

#[component]
pub fn GenericI32<'a, G: Html>(scope: Scope<'a>, props: GenericI32Props<'a>) -> View<G> {
    let map = props
        .partitioned_samples
        .iter()
        .map(|(_, sample_values)| sample_values.iter())
        .flatten()
        .map(|(_, value)| value);
    let max = *map.clone().max().unwrap();
    let min = *map.min().unwrap();
    let bins: Vec<Vec<(PartitionDefnData, Vec<SampleId>)>> = (min..(max + 1))
        .into_iter()
        .map(|dv| {
            props
                .partitioned_samples
                .iter()
                .map(|(partition_defn_data, _)| todo!())
                .collect()
        })
        .collect();
    todo!()
}
