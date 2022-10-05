use super::*;

#[derive(Prop)]
pub struct Contour2dProps<'a> {
    points: &'a [Point2dData],
}

impl<'a> Contour2dProps<'a> {
    fn edges(&self) -> Vec<(Point2dData, Point2dData)> {
        let closed: bool = true;
        let mut edges = vec![];
        for i in 0..(self.points.len() - 1) {
            edges.push((self.points[i], self.points[i + 1]));
        }
        if (closed) {
            edges.push((self.points[self.points.len() - 1], self.points[0]));
        }
        edges
    }
}

#[component]
pub fn Contour2d<'a, G: Html>(scope: Scope<'a>, props: Contour2dProps<'a>) -> View<G> {
    let edges = props.edges();
    view! {
        scope,
        (View::new_fragment(
            edges.iter().map(
                |(from, to)| view!{
                    scope,
                    LineSegment2d {
                        start: *from,
                        end: *to,
                        line_width: 0.05,
                        fill: "red".into()
                    }
                }
            ).collect()
        ))
        (View::new_fragment(
            props.points.iter().map(
                |point| view!{
                    scope,
                    Point2d {
                        point
                    }
                }
            ).collect()
        ))
    }
}
