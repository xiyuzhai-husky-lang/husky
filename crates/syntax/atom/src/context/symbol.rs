use vm::MemberLiason;

use super::*;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ident: CustomIdentifier,
    pub kind: SymbolKind,
}

impl Symbol {
    pub fn variable(ranged_ident: RangedCustomIdentifier) -> Self {
        Self {
            ident: ranged_ident.ident.into(),
            kind: SymbolKind::Variable {
                init_range: ranged_ident.range,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
        opt_this_liason: Option<InputLiason>,
    },
    ThisField {
        opt_this_ty: Option<EntityRoutePtr>,
        opt_field_ty: Option<RangedEntityRoute>,
        field_liason: MemberLiason,
    },
}
