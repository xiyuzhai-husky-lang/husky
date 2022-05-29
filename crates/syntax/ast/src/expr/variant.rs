use crate::*;
use entity_route::GenericArgument;
use entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use text::RangedCustomIdentifier;
use text::Row;
use vm::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprVariant {
    Variable {
        varname: CustomIdentifier,
        init_range: TextRange,
    },
    FrameVariable {
        varname: CustomIdentifier,
        init_range: TextRange,
    },
    This {
        opt_ty: Option<EntityRoutePtr>,
        opt_liason: Option<InputLiason>,
    },
    Unrecognized(CustomIdentifier),
    Entity {
        route: EntityRoutePtr,
        kind: EntityKind,
    },
    CopyableLiteral(CopyableValue),
    Bracketed(RawExprIdx),
    Opn {
        opn_variant: RawOpnVariant,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(RangedCustomIdentifier, Option<RangedEntityRoute>)>,
        RawExprIdx,
    ),
}
