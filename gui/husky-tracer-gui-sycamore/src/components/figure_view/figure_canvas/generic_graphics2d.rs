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
    partitioned_samples: Vec<(&'static Partition, Vec<(SampleId, Graphics2dCanvasValue)>)>,
}

#[component]
pub fn GenericGraphics2d<'a, G: Html>(
    visibility: Scope<'a>,
    props: GenericGraphics2dProps<'a>,
) -> View<G> {
    let nline: u32 = props
        .partitioned_samples
        .iter()
        .map(|(partition, _)| partition.ncol + 1)
        .sum();
    let ncol: u32 = props
        .partitioned_samples
        .iter()
        .map(|(partition, _)| partition.ncol)
        .sum();
    let column_dimension = memo!(visibility, move || {
        (props.dimension.cget()
            - (
                nline * GENERIC_SEPARATOR_LINE_WIDTH,
                TITLE_HEIGHT + GENERIC_BOTTOM_SPACE,
            ))
            / (ncol, 1)
    });
    let actual_dimension = memo!(visibility, move || {
        PixelDimension {
            width: column_dimension.cget().width * ncol + nline * GENERIC_SEPARATOR_LINE_WIDTH,
            height: props.dimension.cget().height - GENERIC_BOTTOM_SPACE,
        }
    });
    let ctx = use_dev_context(visibility);
    let presentation_signal = ctx.presentation_signal();
    view! {
        visibility,
        div (
            class="GenericGraphics2dCanvas",
            style=actual_dimension.cget().to_style()
        ) {
            (View::new_fragment(props.partitioned_samples.clone().into_iter().enumerate().map(
                |(idx, (partition, samples))| {
                    view!{
                        visibility,
                        PartitionContent {
                            idx,
                            column_dimension,
                            partition,
                            samples,
                            presentation_signal,
                        }
                    }
                }
            ).collect()))
        }
    }
}
