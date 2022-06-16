mod partition;

use super::*;
use partition::*;

const TITLE_HEIGHT: u32 = 25;

#[derive(Prop)]
pub struct GenericGraphics2dProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: Vec<(PartitionDefnData, Vec<Graphics2dCanvasData>)>,
}

#[component]
pub fn GenericGraphics2d<'a, G: Html>(
    scope: Scope<'a>,
    props: GenericGraphics2dProps<'a>,
) -> View<G> {
    let dimension = props.dimension;
    let column_dimension = memo!(scope, {
        let dimension = dimension.get_cloned();
        PixelDimension {
            height: dimension.height - TITLE_HEIGHT,
            width: dimension.width / 7,
        }
    });
    view! {
        scope,
        div (
            class="GenericGraphics2dCanvas",
            style=props.dimension.get_cloned().to_style()
        ) {
            (View::new_fragment(props.partitioned_samples.iter().map(
                |(partition, samples)| {
                    view!{
                        scope,
                        PartitionCanvas {
                            column_dimension,
                            partition:  partition.clone() ,
                            samples: samples.clone(),
                        }
                    }
                }
            ).collect()))
        }
    }
}
