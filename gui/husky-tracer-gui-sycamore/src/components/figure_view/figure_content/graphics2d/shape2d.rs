mod arrow2d;
mod contour2d;
mod line_segment2d;
mod point2d;

use super::*;
use arrow2d::*;
use contour2d::*;
use line_segment2d::*;
use point2d::*;

#[derive(Prop)]
pub struct Shape2dProps {
    data: Shape2dData,
}

#[component]
pub fn Shape2d<'a, G: Html>(scope: Scope<'a>, props: Shape2dProps) -> View<G> {
    match props.data {
        Shape2dData::Arrow2d { from, to } => view! {
            scope,
            Arrow2d {
                from,
                to
            }
        },
        Shape2dData::Point2d { point } => view! {
            scope,
            Point2d {
                point
            }
        },
        Shape2dData::Contour { points } => view! {
            scope,
            Contour2d {
                points
            }
        },
        Shape2dData::LineSegment { start, end } => view! {
            scope,
            LineSegment2d {
                start,
                end,
                line_width: 0.1,
                fill: "orange".into()
            }
        },
        Shape2dData::Group { shapes } => view! {
            scope,
            g {
                (View::new_fragment(shapes.iter().map(|data|{
                    view! {
                        scope,
                        Shape2d {
                            data: data.clone()
                        }
                    }
                }).collect()))
            }
        },
    }
}
