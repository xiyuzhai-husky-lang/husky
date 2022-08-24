use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Symbol {
    pub init_ident: RangedCustomIdentifier,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn variable(init_ident: RangedCustomIdentifier) -> Self {
        Self {
            init_ident: init_ident,
            kind: SymbolKind::Variable {
                init_range: init_ident.range,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolKind {
    EntityRoute(EntityRoutePtr),
    Variable {
        init_range: TextRange,
    },
    FrameVariable {
        init_range: TextRange,
    },
    Unrecognized(CustomIdentifier),
    ThisValue {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_this_liason: Option<ParameterModifier>,
    },
    ThisMethod,
    ThisField {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_field_ty: Option<RangedEntityRoute>,
        field_liason: MemberLiason,
    },
}
