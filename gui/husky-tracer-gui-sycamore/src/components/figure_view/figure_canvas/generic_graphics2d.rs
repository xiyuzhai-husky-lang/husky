mod partition;
mod partition_control;

use super::*;
use partition::*;
use partition_control::*;

const TITLE_HEIGHT: u32 = 25;

#[derive(Prop)]
pub struct GenericGraphics2dProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: &'a [(PartitionDefnData, Vec<(SampleId, Graphics2dCanvasData)>)],
}

#[component]
pub fn GenericGraphics2d<'a, G: Html>(
    scope: Scope<'a>,
    props: GenericGraphics2dProps<'a>,
) -> View<G> {
    let dimension = props.dimension;
    let column_dimension = memo!(scope, || { dimension.cget() / (7, 1) - (2, TITLE_HEIGHT) });
    view! {
        scope,
        div (
            class="GenericGraphics2dCanvas",
            style=props.dimension.cget().to_style()
        ) {
            (View::new_fragment(props.partitioned_samples.iter().enumerate().map(
                |(idx,(partition, samples))| {
                    view!{
                        scope,
                        PartitionContent {
                            idx,
                            column_dimension,
                            partition,
                            samples,
                        }
                    }
                }
            ).collect()))
        }
    }
}
