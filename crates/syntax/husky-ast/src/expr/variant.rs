use crate::*;
use husky_entity_route::SpatialArgument;
use husky_entity_route::{EntityKind, EntityRoutePtr, RangedEntityRoute};
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_text::RangedCustomIdentifier;
use husky_text::Row;

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
        opt_this_liason: Option<ParameterModifier>,
    },
    ThisField {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterModifier>,
        field_ident: RangedCustomIdentifier,
        field_liason: MemberModifier,
        opt_field_ty: Option<RangedEntityRoute>,
    },
    Unrecognized(CustomIdentifier),
    Entity {
        route: EntityRoutePtr,
        kind: EntityKind,
    },
    PrimitiveLiteral(PrimitiveLiteralData),
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
