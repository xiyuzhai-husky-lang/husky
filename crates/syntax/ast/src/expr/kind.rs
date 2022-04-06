use std::sync::Arc;

use atom::AtomKind;

use text::Row;
use vm::PrimitiveValue;
use word::WordPtr;

use crate::*;
use entity_route::{EntityRouteKind, EntityRoutePtr, RangedEntityRoute, RawEntityKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprKind {
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    This {
        ty: Option<EntityRoutePtr>,
    },
    Unrecognized(CustomIdentifier),
    Scope {
        scope: EntityRoutePtr,
        kind: RawEntityKind,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn {
        opr: Opr,
        opds: RawExprRange,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<RangedEntityRoute>)>,
        RawExprIdx,
    ),
}
