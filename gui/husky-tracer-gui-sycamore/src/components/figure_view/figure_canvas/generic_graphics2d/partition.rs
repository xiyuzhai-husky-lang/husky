use super::*;

#[derive(Prop)]
pub struct PartitionCanvasProps<'a> {
    column_dimension: &'a ReadSignal<PixelDimension>,
    partition: &'a PartitionDefnData,
    samples: &'a [(SampleId, Graphics2dCanvasData)],
}

#[component]
pub fn PartitionCanvas<'a, G: Html>(scope: Scope<'a>, props: PartitionCanvasProps<'a>) -> View<G> {
    let column_dimension = props.column_dimension;
    let dimension = memo!(scope, move || {
        column_dimension.cget() * (props.partition.ncol, 1) + (0, TITLE_HEIGHT)
    });
    let title_dimension = memo!(scope, move || {
        PixelDimension {
            height: TITLE_HEIGHT,
            width: props.partition.ncol * column_dimension.cget().width,
        }
    });
    let samples_canvas_dimension = memo!(scope, move || {
        let column_dimension = column_dimension.cget();
        PixelDimension {
            height: column_dimension.height,
            width: props.partition.ncol * column_dimension.width,
        }
    });
    let sample_wrapper_dimension =
        memo!(scope, move || { column_dimension.cget() / (1, 5) - (2, 2) });
    let sample_graphics2d_dimension =
        memo!(scope, move || { column_dimension.cget() / (1, 5) - (2, 4) });
    view! {
        scope,
        div (
            class="PartitionCanvas",
            style=dimension.cget().to_style(),
        ) {
            div (
                class="PartitionTitle",
                style=title_dimension.cget().to_style(),
            ) {
                div (class = "PartitionTitleLeft") {
                    button (class = "Add") {
                        svg (
                            stroke="currentColor",
                            fill="currentColor",
                            stroke-width="0",
                            viewBox="0 0 24 24",
                            height="1em",
                            width="1em",
                            xmlns="http://www.w3.org/2000/svg"
                        ) {
                            path (
                                fill="none",
                                stroke="#000",
                                stroke-width="2",
                                d="M12,22 L12,2 M2,12 L22,12"
                            )
                        }
                    }
                }
                div (class = "PartitionTitleRight") {
                    button (class = "PartionTitleRightControl PartitionClose") {
                        "X"
                    }
                    button (class = "PartionTitleRightControl PartitionExpand") {
                        svg (
                            stroke="currentColor",
                            fill="currentColor",
                            stroke-width="0",
                            viewBox="0 0 16 16",
                            height="1em",
                            width="1em",
                            xmlns="http://www.w3.org/2000/svg"
                        ) {
                            path (
                                fill-rule="evenodd",
                                d="M3.646 1.646a.5.5 0 01.708 0l6 6a.5.5 0 010 .708l-6 6a.5.5 0 01-.708-.708L9.293 8 3.646 2.354a.5.5 0 010-.708z",
                                clip-rule="evenodd"
                            )
                            path (
                                fill-rule="evenodd",
                                d="M7.646 1.646a.5.5 0 01.708 0l6 6a.5.5 0 010 .708l-6 6a.5.5 0 01-.708-.708L13.293 8 7.646 2.354a.5.5 0 010-.708z",
                                clip-rule="evenodd"
                            )
                        }
                    }
                    button (class = "PartionTitleRightControl PartitionShrink") {
                        svg (
                            stroke="currentColor",
                            fill="currentColor",
                            stroke-width="0",
                            viewBox="0 0 16 16",
                            height="1em",
                            width="1em",
                            xmlns="http://www.w3.org/2000/svg"
                        ) {
                            path (
                                fill-rule="evenodd",
                                d="M8.354 1.646a.5.5 0 010 .708L2.707 8l5.647 5.646a.5.5 0 01-.708.708l-6-6a.5.5 0 010-.708l6-6a.5.5 0 01.708 0z",
                                clip-rule="evenodd"
                            )
                            path (
                                fill-rule="evenodd",
                                d="M12.354 1.646a.5.5 0 010 .708L6.707 8l5.647 5.646a.5.5 0 01-.708.708l-6-6a.5.5 0 010-.708l6-6a.5.5 0 01.708 0z",
                                clip-rule="evenodd"
                            )
                        }
                    }
                }
            }
            div (
                class="PartitionSamples",
                style=samples_canvas_dimension.cget().to_style(),
            ) {
                (View::new_fragment(
                    props.samples.iter().map(|(sample_id, sample_visual)|
                        view! {
                            scope,
                            div (
                                class="SampleWrapper",
                                style=sample_wrapper_dimension.cget().to_style(),
                            ) {
                                Graphics2dCanvas {
                                    dimension: sample_graphics2d_dimension,
                                    image_layers: &sample_visual.image_layers ,
                                    shapes: &sample_visual.shapes ,
                                    xrange: sample_visual.xrange,
                                    yrange: sample_visual.yrange,
                                }
                            }
                        }
                    ).collect()
                ))
            }
        }
    }
}
