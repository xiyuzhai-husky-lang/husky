use crate::{
    figure::{Color, ImageProps, Plot2dKind, Point2d, PointGroup, Shape, ShapeGroup},
    *,
};

macro_rules! kw {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Keyword,
            value: $value,
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Keyword,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
macro_rules! ident {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: $value,
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
macro_rules! spl {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Special,
            value: $value,
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Special,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
macro_rules! scp {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Scope,
            value: $value,
            spaces_before: None,
        }
    }};
}
macro_rules! fade {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Fade,
            value: $value,
            spaces_before: None,
        }
    }};
}

pub fn root_traces() -> Vec<Arc<Trace>> {
    vec![Arc::new(Trace {
        id: 0,
        parent: None,
        tokens: vec![
            kw!("let"),
            ident!("a"),
            spl!("="),
            scp!("f"),
            spl!("(", 0),
            ident!("c", 0),
            spl!("+"),
            ident!("b"),
            spl!(")", 0),
            fade!("="),
            fade!("a plot"),
        ],
    })]
}
pub fn subtraces(id: usize) -> Vec<Arc<Trace>> {
    if id == 0 {
        vec![
            Arc::new(Trace {
                id: 1,
                parent: Some(0),
                tokens: vec![
                    kw!("let", 4),
                    ident!("a"),
                    spl!("="),
                    scp!("f"),
                    spl!("(", 0),
                    ident!("c", 0),
                    spl!("+"),
                    ident!("b"),
                    spl!(")", 0),
                    fade!("="),
                    fade!("a plot"),
                ],
            }),
            Arc::new(Trace {
                id: 2,
                parent: Some(0),
                tokens: vec![
                    kw!("let", 4),
                    ident!("a"),
                    spl!("="),
                    scp!("f"),
                    spl!("(", 0),
                    ident!("c", 0),
                    spl!("+"),
                    ident!("b"),
                    spl!(")", 0),
                    fade!("="),
                    fade!("a plot"),
                ],
            }),
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
