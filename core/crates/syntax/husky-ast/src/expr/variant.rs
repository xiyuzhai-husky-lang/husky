use crate::*;
use husky_entity_route_syntax::SpatialArgument;
use husky_entity_route_syntax::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use husky_text::Row;
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
        opt_this_liason: Option<ParameterLiason>,
    },
    ThisField {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterLiason>,
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
