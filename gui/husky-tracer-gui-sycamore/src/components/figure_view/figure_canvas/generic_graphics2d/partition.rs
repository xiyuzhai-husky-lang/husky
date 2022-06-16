use super::*;

#[derive(Prop)]
pub struct PartitionCanvasProps<'a> {
    column_dimension: &'a ReadSignal<PixelDimension>,
    partition: PartitionDefnData,
    samples: Vec<Graphics2dCanvasData>,
}

#[component]
pub fn PartitionCanvas<'a, G: Html>(scope: Scope<'a>, props: PartitionCanvasProps<'a>) -> View<G> {
    let column_dimension = props.column_dimension;
    let dimension = memo!(scope, {
        let column_dimension = column_dimension.get_cloned();
        PixelDimension {
            height: column_dimension.height + TITLE_HEIGHT,
            width: props.partition.ncol * column_dimension.width,
        }
    });
    let title_dimension = memo!(scope, {
        let column_dimension = column_dimension.get_cloned();
        PixelDimension {
            height: TITLE_HEIGHT,
            width: props.partition.ncol * column_dimension.width,
        }
    });
    let samples_canvas_dimension = memo!(scope, {
        let column_dimension = column_dimension.get_cloned();
        PixelDimension {
            height: column_dimension.height,
            width: props.partition.ncol * column_dimension.width,
        }
    });
    let sample_wrapper_dimension = memo!(scope, {
        let column_dimension = column_dimension.get_cloned();
        PixelDimension {
            height: column_dimension.height / 5 - 2,
            width: column_dimension.width - 2,
        }
    });
    let sample_graphics2d_dimension = memo!(scope, {
        let column_dimension = column_dimension.get_cloned();
        PixelDimension {
            height: column_dimension.height / 5 - 4,
            width: column_dimension.width - 2,
        }
    });
    view! {
        scope,
        div (
            class="PartitionCanvas",
            style=dimension.get_cloned().to_style(),
        ) {
            div (
                class="PartitionTitle",
                style=title_dimension.get_cloned().to_style(),
            )
            div (
                class="PartitionSamples",
                style=samples_canvas_dimension.get_cloned().to_style(),
            ) {
                (View::new_fragment(
                    props.samples.iter().map(|sample|
                        view! {
                            scope,
                            div (
                                class="SampleWrapper",
                                style=sample_wrapper_dimension.get_cloned().to_style(),
                            ) {
                                Graphics2dCanvas {
                                    dimension: sample_graphics2d_dimension,
                                    image_layers: Rc::new(sample.image_layers.clone()),
                                    shapes: Rc::new(sample.shapes.clone()),
                                    xrange: sample.xrange,
                                    yrange: sample.yrange,
                                }
                            }
                        }
                    ).collect()
                ))
            }
        }
    }
}
