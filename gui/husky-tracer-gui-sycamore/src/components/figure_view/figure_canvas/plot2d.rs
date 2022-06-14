use super::*;

pub struct Plot2dCanvasProps {
    plot_kind: Plot2dKind,
    point_groups: Vec<Point2dGroup>,
    xrange: (f32, f32),
    yrange: (f32, f32),
}

#[component]
fn Plot2dCanvas<'a, G: Html>(cx: Scope<'a>, props: Plot2dCanvasProps) -> View<G> {
    match props.plot_kind {
        Plot2dKind::Scatter => todo!(),
    }
}
