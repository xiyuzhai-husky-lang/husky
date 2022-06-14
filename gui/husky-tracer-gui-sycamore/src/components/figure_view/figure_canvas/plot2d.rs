use super::*;

#[derive(Prop)]
pub struct Plot2dCanvasProps<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    plot_kind: Plot2dKind,
    point_groups: Vec<Point2dGroup>,
    xrange: (f32, f32),
    yrange: (f32, f32),
}

#[component]
pub fn Plot2dCanvas<'a, G: Html>(cx: Scope<'a>, props: Plot2dCanvasProps<'a>) -> View<G> {
    match props.plot_kind {
        Plot2dKind::Scatter => todo!(),
    }
}
