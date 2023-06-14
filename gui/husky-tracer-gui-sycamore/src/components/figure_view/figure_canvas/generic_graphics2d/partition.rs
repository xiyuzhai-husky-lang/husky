use super::*;

#[derive(Prop)]
pub struct PartitionContentProps<'a> {
    idx: usize,
    column_dimension: &'a ReadSignal<PixelDimension>,
    partition: &'static Partition,
    samples: Vec<(SampleId, Graphics2dCanvasValue)>,
    presentation_signal: &'a ReadSignal<Presentation>,
}

#[component]
pub fn PartitionContent<'a, G: Html>(
    visibility: Scope<'a>,
    props: PartitionContentProps<'a>,
) -> View<G> {
    let column_dimension = props.column_dimension;
    let dimension = memo!(visibility, move || {
        column_dimension.cget() * (props.partition.ncol, 1)
            + (
                (props.partition.ncol + 1) * GENERIC_SEPARATOR_LINE_WIDTH,
                TITLE_HEIGHT,
            )
    });
    let top_bar_dimension = memo!(visibility, move || {
        PixelDimension {
            height: TITLE_HEIGHT,
            width: dimension.cget().width,
        }
    });
    let samples_canvas_dimension = memo!(visibility, move || {
        let column_dimension = column_dimension.cget();
        PixelDimension {
            height: column_dimension.height,
            width: dimension.cget().width,
        }
    });
    let sample_wrapper_dimension = memo!(visibility, move || {
        column_dimension.cget() / (1, 5) - (2, 2)
    });
    let sample_graphics2d_dimension = memo!(visibility, move || {
        column_dimension.cget() / (1, 5) - (2, 4)
    });
    let partition = format!("{}", props.partition);
    let presentation_signal = props.presentation_signal;
    let ctx = use_dev_context(visibility);
    view! {
        visibility,
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
                    props.samples.clone().into_iter().map(|(sample_id, sample_canvas_value)|{
                        view! {
                            visibility,
                            div (
                                class=if presentation_signal.get().sample_id() == sample_id { "SampleWrapper anchored" } else { "SampleWrapper" },
                                style=sample_wrapper_dimension.cget().to_style(),
                                on:click=ctx.set_sample_id_handler(sample_id),
                            ) {
                                Graphics2dCanvas {
                                    dimension: sample_graphics2d_dimension,
                                    image_layers: sample_canvas_value.image_layers(),
                                    shapes: sample_canvas_value.shapes(),
                                    xrange: sample_canvas_value.xrange,
                                    yrange: sample_canvas_value.yrange,
                                }
                            }
                        }
                    }).collect()
                ))
            }
        }
    }
}
