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
pub struct Shape2dProps<'a> {
    data: &'a Shape2dData,
}

#[component]
pub fn Shape2d<'a, G: Html>(visibility: Scope<'a>, props: Shape2dProps<'a>) -> View<G> {
    match props.data {
        Shape2dData::Arrow2d { from, to } => view! {
            visibility,
            Arrow2d {
                from,
                to
            }
        },
        Shape2dData::Point2d { point } => view! {
            visibility,
            Point2d {
                point
            }
        },
        Shape2dData::Contour { points } => view! {
            visibility,
            Contour2d {
                points
            }
        },
        Shape2dData::LineSegment { start, end } => view! {
            visibility,
            LineSegment2d {
                start: *start,
                end: *end,
                line_width: 0.1,
                fill: "orange".into()
            }
        },
        Shape2dData::Group { shapes } => view! {
            visibility,
            g {
                (View::new_fragment(shapes.iter().map(|data|{
                    view! {
                        visibility,
                        Shape2d {
                            data
                        }
                    }
                }).collect()))
            }
        },
    }
}
