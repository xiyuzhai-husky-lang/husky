use crate::{
    figure::{Color, ImageProps, Plot2dKind, Point2d, PointGroup, Shape, ShapeGroup},
    token::*,
    *,
};

pub fn root_traces() -> Vec<Arc<Trace>> {
    vec![Trace::mock(
        0,
        None,
        vec![
            keyword!("let"),
            ident!("a"),
            special!("="),
            scp!("f"),
            special!("(", 0),
            ident!("c", 0),
            special!("+"),
            ident!("b"),
            special!(")", 0),
            fade!("="),
            fade!("a plot"),
        ],
    )]
}
pub fn subtraces(id: usize) -> Vec<Arc<Trace>> {
    if id == 0 {
        vec![
            Trace::mock(
                1,
                Some(0),
                vec![
                    keyword!("let", 4),
                    ident!("a"),
                    special!("="),
                    scp!("f"),
                    special!("(", 0),
                    ident!("c", 0),
                    special!("+"),
                    ident!("b"),
                    special!(")", 0),
                    fade!("="),
                    fade!("a plot"),
                ],
            ),
            Trace::mock(
                2,
                Some(0),
                vec![
                    keyword!("let", 4),
                    ident!("a"),
                    special!("="),
                    scp!("f"),
                    special!("(", 0),
                    ident!("c", 0),
                    special!("+"),
                    ident!("b"),
                    special!(")", 0),
                    fade!("="),
                    fade!("a plot"),
                ],
            ),
        ]
    } else {
        vec![]
    }
}

pub fn figure(id: usize) -> Option<FigureProps> {
    match id {
        0 => Some(FigureProps::Plot2d {
            plot_kind: Plot2dKind::Scatter,
            groups: vec![
                PointGroup {
                    points: vec![
                        Point2d { x: 0.0, y: 0.2 },
                        Point2d { x: 2.2, y: -1.0 },
                        Point2d { x: 3.2, y: -1.0 },
                        Point2d { x: 4.2, y: -2.0 },
                        Point2d { x: 2.2, y: -1.5 },
                        Point2d { x: 3.2, y: -1.2 },
                        Point2d { x: 3.2, y: -1.2 },
                    ],
                    color: Color::Red,
                },
                PointGroup {
                    points: vec![Point2d { x: 0.1, y: 0.2 }, Point2d { x: 1.2, y: -1.0 }],
                    color: Color::Yellow,
                },
            ],
            xrange: (-5.0, 5.0),
            yrange: (-5.0, 5.0),
        }),
        1 => Some(FigureProps::Graphics2d {
            image: None,
            shape_groups: vec![ShapeGroup {
                shapes: vec![Shape::Arrow {
                    from: Point2d { x: 5.0, y: 6.0 },
                    to: Point2d { x: 16.0, y: 15.0 },
                }],
                line_width: 0.15,
                color: Color::Green,
            }],
            xrange: (0.0, 28.0),
            yrange: (0.0, 28.0),
        }),
        2 => Some(FigureProps::Graphics2d {
            image: Some(ImageProps {
                data: vec![],
                original_width: 28,
                original_height: 28,
            }),
            shape_groups: vec![
                ShapeGroup {
                    shapes: vec![Shape::Arrow {
                        from: Point2d { x: 5.0, y: 6.0 },
                        to: Point2d { x: 16.0, y: 15.0 },
                    }],
                    line_width: 0.15,
                    color: Color::Blue,
                },
                ShapeGroup {
                    shapes: vec![Shape::Arrow {
                        from: Point2d { x: 16.0, y: 6.0 },
                        to: Point2d { x: 5.0, y: 15.0 },
                    }],
                    line_width: 0.15,
                    color: Color::Red,
                },
            ],
            xrange: (0.0, 28.0),
            yrange: (0.0, 28.0),
        }),
        _ => None,
    }
}
