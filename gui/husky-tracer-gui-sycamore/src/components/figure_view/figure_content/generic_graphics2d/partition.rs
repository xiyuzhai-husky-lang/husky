use super::*;

#[derive(Prop)]
pub struct PartitionContentProps<'a> {
    idx: usize,
    column_dimension: &'a ReadSignal<PixelDimension>,
    partition: &'a PartitionDefnData,
    samples: &'a [(SampleId, Graphics2dCanvasData)],
    restriction: &'a ReadSignal<Restriction>,
}

#[component]
pub fn PartitionContent<'a, G: Html>(
    scope: Scope<'a>,
    props: PartitionContentProps<'a>,
) -> View<G> {
    let column_dimension = props.column_dimension;
    let dimension = memo!(scope, move || {
        column_dimension.cget() * (props.partition.ncol, 1)
            + (
                (props.partition.ncol + 1) * GENERIC_SEPARATOR_LINE_WIDTH,
                TITLE_HEIGHT,
            )
    });
    let top_bar_dimension = memo!(scope, move || {
        PixelDimension {
            height: TITLE_HEIGHT,
            width: dimension.cget().width,
        }
    });
    let samples_canvas_dimension = memo!(scope, move || {
        let column_dimension = column_dimension.cget();
        PixelDimension {
            height: column_dimension.height,
            width: dimension.cget().width,
        }
    });
    let sample_wrapper_dimension =
        memo!(scope, move || { column_dimension.cget() / (1, 5) - (2, 2) });
    let sample_graphics2d_dimension =
        memo!(scope, move || { column_dimension.cget() / (1, 5) - (2, 4) });
    let partition = format!("{}", props.partition);
    let restriction = props.restriction;
    view! {
        scope,
        div (
            class=format!("PartitionContent Index{}",props.idx),
            style=dimension.cget().to_style(),
        ) {
            div (
                class="PartitionTopBar",
                style=top_bar_dimension.cget().to_style(),
            ){
                div (class="PartitionTitle") {
                    (partition)
                }
                PartitionControl {
                    idx: props.idx,
                    dimension: top_bar_dimension
                }
            }
            div (
                class="PartitionSamples",
                style=samples_canvas_dimension.cget().to_style(),
            ) {
                (View::new_fragment(
                    props.samples.iter().map(|(sample_id, sample_visual)|{
                        let anchored = memo!(scope, move || restriction.get().sample_id() == *sample_id);
                        view! {
                            scope,
                            div (
                                class=if *anchored.get() { "SampleWrapper anchored" } else { "SampleWrapper" },
                                style=sample_wrapper_dimension.cget().to_style(),
                            ) {
                                Graphics2dCanvas {
                                    dimension: sample_graphics2d_dimension,
                                    image_layers: memo!(scope, move || sample_visual.image_layers()), /* todo */
                                    shapes: memo!(scope, move || sample_visual.shapes()), /* todo */
                                    xrange: sample_visual.xrange,
                                    yrange: sample_visual.yrange,
                                }
                            }
                        }
                    }).collect()
                ))
            }
        }
    }
}
