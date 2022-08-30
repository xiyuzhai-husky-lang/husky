mod scale;

use super::*;
use scale::*;

#[derive(Prop)]
pub struct GenericF32Props<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: &'a [(PartitionDefnData, Vec<(SampleId, f32)>)],
}

#[component]
pub fn GenericF32<'a, G: Html>(scope: Scope<'a>, props: GenericF32Props<'a>) -> View<G> {
    let scale = GenericF32Scale::new(props.partitioned_samples);
    let focus = create_signal(scope, None);
    let mut points = vec![];
    for (i, (_partition, samples)) in props.partitioned_samples.iter().enumerate() {
        for (sample_id, value) in samples.iter() {
            let circle = scale.circle(i, *value);
            points.push(view! {scope, GenericF32Point {
                focus,
                sample_id: *sample_id,
                circle
            }})
        }
    }
    let points = View::new_fragment(points);
    view! {
        scope,
        div (
            class="GenericF32",
            style=props.dimension.cget().to_style(),
        ) {
            div (class="GenericF32PlotRegion") {
                svg (
                    class="GenericF32PlotRegion",
                    viewBox=scale.svg_view_box()
                ) {
                    (points)
                }
            }
            div (class="GenericF32VisualRegion") {}
        }
    }
}

#[derive(Prop)]
pub struct GenericF32PointProps<'a> {
    focus: &'a Signal<Option<SampleId>>,
    sample_id: SampleId,
    circle: CircleProps,
}

#[component]
fn GenericF32Point<'a, G: Html>(scope: Scope<'a>, props: GenericF32PointProps<'a>) -> View<G> {
    view! {
        scope,
        circle (
            class=format!(
                "ClassIndex{} {}",
                props.circle.class_index,
                if *props.focus.get() == Some(props.sample_id) {
                    " focused"
                } else {
                    ""
                }
            ),
            on:click=move |_| {
                if *props.focus.get() != Some(props.sample_id) {
                    props.focus.set(Some(props.sample_id))
                } else {
                    props.focus.set(None)
                }
            },
            cx={props.circle.cx},
            cy={props.circle.cy},
            r={props.circle.r}
        )
    }
}
