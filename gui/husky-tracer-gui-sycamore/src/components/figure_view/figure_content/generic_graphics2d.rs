mod config;
mod partition;
mod partition_control;

use super::*;
use config::*;
use partition::*;
use partition_control::*;

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
    let partitioned_samples = props.partitioned_samples;
    let column_dimension = memo!(scope, || {
        let nline: u32 = partitioned_samples
            .iter()
            .map(|(partition, _)| partition.ncol + 1)
            .sum();
        let ncol: u32 = partitioned_samples
            .iter()
            .map(|(partition, _)| partition.ncol)
            .sum();
        (props.dimension.cget()
            - (
                nline * GENERIC_SEPARATOR_LINE_WIDTH,
                TITLE_HEIGHT + GENERIC_BOTTOM_SPACE,
            ))
            / (ncol, 1)
    });
    let actual_dimension = memo!(scope, || {
        let nline: u32 = partitioned_samples
            .iter()
            .map(|(partition, _)| partition.ncol + 1)
            .sum();
        let ncol: u32 = partitioned_samples
            .iter()
            .map(|(partition, _)| partition.ncol)
            .sum();
        PixelDimension {
            width: column_dimension.cget().width * ncol + nline * GENERIC_SEPARATOR_LINE_WIDTH,
            height: props.dimension.cget().height - GENERIC_BOTTOM_SPACE,
        }
    });
    view! {
        scope,
        div (
            class="GenericGraphics2dCanvas",
            style=actual_dimension.cget().to_style()
        ) {
            (View::new_fragment(props.partitioned_samples.iter().enumerate().map(
                |(idx, (partition, samples))| {
                    assert!(samples.len() > 0);
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
