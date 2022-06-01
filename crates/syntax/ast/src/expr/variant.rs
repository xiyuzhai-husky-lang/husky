use crate::*;
use entity_route::SpatialArgument;
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
    ThisValue {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<InputLiason>,
    },
    ThisField {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<InputLiason>,
        field_ident: RangedCustomIdentifier,
        field_liason: MemberLiason,
        opt_field_ty: Option<RangedEntityRoute>,
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
