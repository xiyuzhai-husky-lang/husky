use super::*;
use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Convexity {
    Convex,
    Concave,
}

pub(super) fn left_side_convexity(kind: &AtomKind) -> Convexity {
    match kind {
        AtomKind::Scope(_, _)
        | AtomKind::Variable(_)
        | AtomKind::Literal(_)
        | AtomKind::Prefix(_)
        | AtomKind::ListStart(_, ListStartAttr::None)
        | AtomKind::ListEnd(_, _)
        | AtomKind::LambdaHead(_) => Convexity::Convex,
        AtomKind::Suffix(_)
        | AtomKind::Binary(_)
        | AtomKind::ListStart(_, ListStartAttr::Attach)
        | AtomKind::ListItem => Convexity::Concave,
    }
}

pub(super) fn right_side_convexity(kind: &AtomKind) -> Convexity {
    match kind {
        AtomKind::Scope(_, _)
        | AtomKind::Variable(_)
        | AtomKind::Literal(_)
        | AtomKind::Suffix(_)
        | AtomKind::ListEnd(_, ListEndAttr::None)
        | AtomKind::ListEnd(_, ListEndAttr::Modulo) => Convexity::Convex,
        AtomKind::Prefix(_)
        | AtomKind::Binary(_)
        | AtomKind::ListStart(_, _)
        | AtomKind::ListItem
        | AtomKind::ListEnd(_, ListEndAttr::Attach)
        | AtomKind::LambdaHead(_) => Convexity::Concave,
    }
}

pub(super) fn compatible(left: Convexity, right: Convexity) -> bool {
    match left {
        Convexity::Convex => match right {
            Convexity::Convex => false,
            Convexity::Concave => true,
        },
        Convexity::Concave => match right {
            Convexity::Convex => true,
            Convexity::Concave => false,
        },
    }
}
