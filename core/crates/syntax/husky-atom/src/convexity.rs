use super::*;
use vm::*;

impl AtomVariant {
    pub(super) fn left_convexity(&self) -> Convexity {
        match self {
            AtomVariant::EntityRoute { .. }
            | AtomVariant::Variable { .. }
            | AtomVariant::FrameVariable { .. }
            | AtomVariant::ThisValue { .. }
            | AtomVariant::ThisField { .. }
            | AtomVariant::Unrecognized(_)
            | AtomVariant::PrimitiveLiteral(_)
            | AtomVariant::Prefix(_)
            | AtomVariant::ListStart(_, ListStartAttr::None)
            | AtomVariant::ListEnd(_, _)
            | AtomVariant::LambdaHead(_) => Convexity::Convex,
            AtomVariant::Suffix(_)
            | AtomVariant::FieldAccess(_)
            | AtomVariant::Binary(_)
            | AtomVariant::ListStart(_, ListStartAttr::Attach)
            | AtomVariant::ListStart(_, ListStartAttr::MethodAttach { .. })
            | AtomVariant::ListItem
            | AtomVariant::SilentEnd => Convexity::Concave,
            AtomVariant::BePattern(_) => todo!(),
        }
    }

    pub(super) fn right_side_convexity(&self) -> Convexity {
        match self {
            AtomVariant::EntityRoute { .. }
            | AtomVariant::Variable { .. }
            | AtomVariant::FrameVariable { .. }
            | AtomVariant::ThisValue { .. }
            | AtomVariant::ThisField { .. }
            | AtomVariant::Unrecognized(_)
            | AtomVariant::PrimitiveLiteral(_)
            | AtomVariant::Suffix(_)
            | AtomVariant::FieldAccess(_)
            | AtomVariant::ListEnd(_, ListEndAttr::None)
            | AtomVariant::ListEnd(_, ListEndAttr::Modulo)
            | AtomVariant::SilentEnd => Convexity::Convex,
            AtomVariant::Prefix(_)
            | AtomVariant::Binary(_)
            | AtomVariant::ListStart(_, _)
            | AtomVariant::ListItem
            | AtomVariant::ListEnd(_, ListEndAttr::Attach)
            | AtomVariant::LambdaHead(_) => Convexity::Concave,
            AtomVariant::BePattern(_) => todo!(),
        }
    }
}
