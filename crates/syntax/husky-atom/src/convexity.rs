use husky_word::WordPattern;

use super::*;
impl HuskyAtomVariant {
    pub(super) fn left_convexity(&self) -> Convexity {
        match self {
            HuskyAtomVariant::EntityRoute { .. }
            | HuskyAtomVariant::Variable { .. }
            | HuskyAtomVariant::FrameVariable { .. }
            | HuskyAtomVariant::ThisValue { .. }
            | HuskyAtomVariant::ThisField { .. }
            | HuskyAtomVariant::Unrecognized(_)
            | HuskyAtomVariant::PrimitiveLiteral(_)
            | HuskyAtomVariant::Prefix(_)
            | HuskyAtomVariant::ListStart(_, ListStartAttr::None)
            | HuskyAtomVariant::ListEnd(_, _)
            | HuskyAtomVariant::LambdaHead(_) => Convexity::Convex,
            HuskyAtomVariant::Suffix(_)
            | HuskyAtomVariant::FieldAccess(_)
            | HuskyAtomVariant::Binary(_)
            | HuskyAtomVariant::ListStart(_, ListStartAttr::Attach)
            | HuskyAtomVariant::ListStart(_, ListStartAttr::MethodAttach { .. })
            | HuskyAtomVariant::ListItem
            | HuskyAtomVariant::SilentEnd => Convexity::Concave,
            HuskyAtomVariant::Be => Convexity::Concave,
            HuskyAtomVariant::BePattern(_) => Convexity::Concave,
            HuskyAtomVariant::WordPattern { .. } => Convexity::Convex,
        }
    }

    pub(super) fn right_side_convexity(&self) -> Convexity {
        match self {
            HuskyAtomVariant::EntityRoute { .. }
            | HuskyAtomVariant::Variable { .. }
            | HuskyAtomVariant::FrameVariable { .. }
            | HuskyAtomVariant::ThisValue { .. }
            | HuskyAtomVariant::ThisField { .. }
            | HuskyAtomVariant::Unrecognized(_)
            | HuskyAtomVariant::PrimitiveLiteral(_)
            | HuskyAtomVariant::Suffix(_)
            | HuskyAtomVariant::FieldAccess(_)
            | HuskyAtomVariant::ListEnd(_, ListEndAttr::None)
            | HuskyAtomVariant::ListEnd(_, ListEndAttr::Modulo)
            | HuskyAtomVariant::SilentEnd => Convexity::Convex,
            HuskyAtomVariant::Prefix(_)
            | HuskyAtomVariant::Binary(_)
            | HuskyAtomVariant::ListStart(_, _)
            | HuskyAtomVariant::ListItem
            | HuskyAtomVariant::ListEnd(_, ListEndAttr::Attach)
            | HuskyAtomVariant::LambdaHead(_) => Convexity::Concave,
            HuskyAtomVariant::Be => Convexity::Concave,
            HuskyAtomVariant::BePattern(_) => Convexity::Convex,
            HuskyAtomVariant::WordPattern { patt } => match patt {
                WordPattern::Some => Convexity::WordPatternAny,
                WordPattern::None => Convexity::Convex,
            },
        }
    }
}
