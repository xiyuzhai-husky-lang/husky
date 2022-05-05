use crate::*;
use entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use text::Row;
use vm::*;
use word::RangedCustomIdentifier;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    FrameVariable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    This {
        opt_ty: Option<EntityRoutePtr>,
        opt_contract: Option<InputContract>,
    },
    Unrecognized(CustomIdentifier),
    Entity {
        route: EntityRoutePtr,
        kind: EntityKind,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn {
        opr: Opr,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
        RawExprIdx,
    ),
}
