use super::*;

#[derive(Prop)]
pub struct TicksProps {
    a: f32,
    b: f32,
}

#[component]
pub fn Ticks<'a, G: Html>(visibility: Scope<'a>, props: TicksProps) -> View<G> {
    let ticks = interpolate(props.a, props.b);
    let ticks = View::new_fragment(
        ticks
            .into_iter()
            .map(|tick| {
                let y = 990.0 - 980.0 * (tick - props.a) / (props.b - props.a);
                view! {
                    visibility,
                    path (
                        d = format!("M43 {} L 53 {}", y, y),
                        stroke="white",
                        fill="transparent",
                        stroke-width="0.5"
                    ) {}
                    text (
                        x="10",
                        y=format!("{}", y + 4.0),
                        class="TickValueText"
                    ) {
                        (tick)
                    }
                }
            })
            .collect(),
    );
    view! {
        visibility,
        path (
            d = "M43 10 L 43 990",
            stroke="white",
            fill="transparent",
            stroke-width="0.5"
        ) {}
        (ticks)
    }
}

fn interpolate(a: f32, b: f32) -> Vec<f32> {
    if a >= b {
        return vec![];
    }
    let l = b - a;
    let d = {
        let d0 = 10f32.powf(l.log10().floor() - 1.0);
        let r = l / d0;
        if r < 25.5 {
            d0
        } else if r < 50.5 {
            2.0 * d0
        } else {
            5.0 * d0
        }
    };
    let mut point = (a / d).ceil() * d;
    let mut points: Vec<f32> = vec![];
    while point < b {
        points.push(point);
        point += d
    }
    points
}

#[test]
fn interpolate_works() {
    assert_eq!(interpolate(0.0, 2.3).len(), 23);
}
