mod scale;
mod ticks;

use super::*;
use scale::*;
use ticks::*;

#[derive(Prop)]
pub struct GenericF32Props<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: &'static [(Partition, Vec<(SampleId, f32)>)],
    image_layers: Vec<&'static ImageLayerData>,
    shapes: Vec<&'static Shape2dData>,
}

#[component]
pub fn GenericF32<'a, G: Html>(visibility: Scope<'a>, props: GenericF32Props<'a>) -> View<G> {
    let scale = GenericF32Scale::new(props.partitioned_samples);
    let a = scale.value_min;
    let b = scale.value_max;
    let mut points = vec![];
    for (i, (_partition, samples)) in props.partitioned_samples.iter().enumerate() {
        for (sample_id, value) in samples.iter() {
            let circle = scale.circle(i, *value);
            points.push(view! {visibility, GenericF32Point {
                sample_id: *sample_id,
                circle
            }})
        }
    }
    let points = View::new_fragment(points);
    let dimension = props.dimension;
    let generic_f32_visual_region_dimension =
        memo!(visibility, move || dimension.cget() * (7, 1) / (10, 1));
    let ctx = use_dev_context(visibility);
    view! {
        visibility,
        div (
            class="GenericF32",
            style=props.dimension.cget().to_style(),
        ) {
            div (class="GenericF32PlotRegion") {
                svg (
                    class="GenericF32PlotRegion",
                    viewBox=scale.svg_view_box()
                ) {
                    Ticks {
                        a,
                        b,
                    }
                    (points)
                }
            }
            div (class="GenericF32VisualRegion") {
                Graphics2dCanvas {
                    dimension: generic_f32_visual_region_dimension,
                    image_layers: props.image_layers,
                    shapes: props.shapes,
                    xrange: (0.0, 28.0), // ad hoc
                    yrange: (0.0, 28.0), // ad hoc
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct GenericF32PointProps {
    sample_id: SampleId,
    circle: CircleProps,
}

#[component]
fn GenericF32Point<'a, G: Html>(visibility: Scope<'a>, props: GenericF32PointProps) -> View<G> {
    let ctx = use_dev_context(visibility);
    let presentation_signal = ctx.presentation_signal();
    view! {
        visibility,
        circle (
            class=format!(
                "ClassIndex{} {}",
                props.circle.class_index,
                if presentation_signal.get().sample_id() == props.sample_id {
                    " focused"
                } else {
                    ""
                }
            ),
            on:click=ctx.set_sample_id_handler(props.sample_id),
            cx={props.circle.cx},
            cy={props.circle.cy},
            r={props.circle.r}
        )
    }
}
