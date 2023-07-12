mod context_mut;
mod map;
mod ordered_map;
mod region;

pub use self::context_mut::*;
pub use self::map::*;
pub use self::ordered_map::*;
pub use self::region::*;

use idx_arena::ordered_map::ArenaOrderedMap;

use crate::*;
use husky_entity_tree::{CratePrelude, ModuleSymbolContext, PreludeResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum Symbol {
    PrincipalEntity(PrincipalEntityPath),
    Inherited(InheritedSymbolIdx, InheritedSymbolKind),
    Local(CurrentSymbolIdx, CurrentSymbolKind),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum ImplicitParameterSymbol {
    Lifetime { label_token: LifetimeLabelToken },
    Type { ident_token: IdentToken },
    Const {},
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct InheritedSymbol {
    parent_symbol_idx: ParentSymbolIdx,
    modifier: SymbolModifier,
    kind: InheritedSymbolKind,
}

impl InheritedSymbol {
    pub fn kind(&self) -> InheritedSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.kind {
            InheritedSymbolKind::ImplicitParameter(kind) => match kind {
                InheritedImplicitParameterSymbol::Lifetime { label } => None,
                InheritedImplicitParameterSymbol::Type { ident }
                | InheritedImplicitParameterSymbol::Constant { ident } => Some(ident),
            },
            InheritedSymbolKind::ExplicitParameter { ident } => Some(ident),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum InheritedSymbolKind {
    ImplicitParameter(InheritedImplicitParameterSymbol),
    ExplicitParameter { ident: Ident },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum InheritedImplicitParameterSymbol {
    Lifetime { label: Label },
    Type { ident: Ident },
    Constant { ident: Ident },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct CurrentSymbol {
    modifier: SymbolModifier,
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    variant: CurrentSymbolVariant,
}

impl CurrentSymbol {
    pub fn new(
        pattern_expr_region: &PatternExprRegion,
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        variant: CurrentSymbolVariant,
    ) -> Self {
        Self {
            modifier: variant.modifier(pattern_expr_region),
            access_start,
            access_end,
            variant,
        }
    }

    pub fn modifier(&self) -> SymbolModifier {
        self.modifier
    }

    pub fn kind(&self) -> CurrentSymbolKind {
        self.variant.kind()
    }

    pub fn variant(&self) -> &CurrentSymbolVariant {
        &self.variant
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.variant {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant:
                    CurrentImplicitParameterSymbol::Type { ident_token }
                    | CurrentImplicitParameterSymbol::Constant { ident_token, .. },
            }
            | CurrentSymbolVariant::ExplicitVariadicParameter { ident_token, .. } => {
                Some(ident_token.ident())
            }
            CurrentSymbolVariant::ExplicitRegularParameter { ident, .. }
            | CurrentSymbolVariant::LetVariable { ident, .. }
            | CurrentSymbolVariant::FrameVariable { ident, .. } => Some(ident),
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime { .. },
            } => None,
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum CurrentSymbolKind {
    ImplicitParameter {
        implicit_parameter_kind: CurrentImplicitParameterSymbolKind,
    },
    ExplicitRegularParameter {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    ExplicitVariadicParameter {
        ident_token: IdentToken,
    },
    LetVariable {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    FrameVariable(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum CurrentImplicitParameterSymbolKind {
    Type { ident_token: IdentToken },
    Lifetime { label_token: LifetimeLabelToken },
    Constant { ident_token: IdentToken },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum CurrentSymbolVariant {
    ImplicitParameter {
        implicit_parameter_variant: CurrentImplicitParameterSymbol,
    },
    ExplicitRegularParameter {
        ident: Ident,
        pattern_symbol_idx: PatternSymbolIdx,
    },
    ExplicitVariadicParameter {
        symbol_modifier_keyword_group: Option<SymbolModifierKeywordGroup>,
        ident_token: IdentToken,
    },
    LetVariable {
        ident: Ident,
        pattern_symbol_idx: PatternSymbolIdx,
    },
    FrameVariable {
        ident: Ident,
        expr_idx: ExprIdx,
    },
}

impl CurrentSymbolVariant {
    fn modifier(&self, pattern_expr_region: &PatternExprRegion) -> SymbolModifier {
        match self {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => SymbolModifier::Const,
            CurrentSymbolVariant::ExplicitRegularParameter {
                pattern_symbol_idx, ..
            }
            | CurrentSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => pattern_expr_region.pattern_symbol_modifier(*pattern_symbol_idx),
            CurrentSymbolVariant::ExplicitVariadicParameter {
                symbol_modifier_keyword_group,
                ..
            } => SymbolModifier::new(*symbol_modifier_keyword_group),
            CurrentSymbolVariant::FrameVariable { ident, expr_idx } => SymbolModifier::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
#[non_exhaustive]
pub enum CurrentImplicitParameterSymbol {
    Lifetime {
        label_token: LifetimeLabelToken,
    },
    Type {
        ident_token: IdentToken,
    },
    Constant {
        ident_token: IdentToken,
        ty_expr_idx: ExprIdx,
    },
}

impl CurrentImplicitParameterSymbol {
    fn bequeath(&self) -> InheritedImplicitParameterSymbol {
        match self {
            CurrentImplicitParameterSymbol::Lifetime { label_token } => {
                InheritedImplicitParameterSymbol::Lifetime {
                    label: label_token.label(),
                }
            }
            CurrentImplicitParameterSymbol::Type { ident_token } => {
                InheritedImplicitParameterSymbol::Type {
                    ident: ident_token.ident(),
                }
            }
            CurrentImplicitParameterSymbol::Constant {
                ident_token,
                ty_expr_idx,
            } => InheritedImplicitParameterSymbol::Constant {
                ident: ident_token.ident(),
            },
        }
    }
}

impl CurrentSymbolVariant {
    pub fn kind(&self) -> CurrentSymbolKind {
        match self {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => CurrentSymbolKind::ImplicitParameter {
                implicit_parameter_kind: implicit_parameter_variant.kind(),
            },
            CurrentSymbolVariant::ExplicitRegularParameter {
                pattern_symbol_idx, ..
            } => CurrentSymbolKind::ExplicitRegularParameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => CurrentSymbolKind::LetVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSymbolVariant::FrameVariable { expr_idx, .. } => {
                CurrentSymbolKind::FrameVariable(*expr_idx)
            }
            CurrentSymbolVariant::ExplicitVariadicParameter { ident_token, .. } => {
                CurrentSymbolKind::ExplicitVariadicParameter {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

impl CurrentImplicitParameterSymbol {
    fn kind(&self) -> CurrentImplicitParameterSymbolKind {
        match self {
            CurrentImplicitParameterSymbol::Type { ident_token } => {
                CurrentImplicitParameterSymbolKind::Type {
                    ident_token: *ident_token,
                }
            }
            CurrentImplicitParameterSymbol::Lifetime { label_token } => {
                CurrentImplicitParameterSymbolKind::Lifetime {
                    label_token: *label_token,
                }
            }
            CurrentImplicitParameterSymbol::Constant { ident_token, .. } => {
                CurrentImplicitParameterSymbolKind::Constant {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

pub type InheritedSymbolArena = Arena<InheritedSymbol>;
pub type InheritedSymbolIdx = ArenaIdx<InheritedSymbol>;
pub type InheritedSymbolIdxRange = ArenaIdxRange<InheritedSymbol>;
pub(crate) type InheritedSymbolMap<V> = ArenaMap<InheritedSymbol, V>;
pub(crate) type InheritedSymbolOrderedMap<V> = ArenaOrderedMap<InheritedSymbol, V>;

pub type CurrentSymbolArena = Arena<CurrentSymbol>;
pub type CurrentSymbolIdx = ArenaIdx<CurrentSymbol>;
pub type CurrentSymbolIdxRange = ArenaIdxRange<CurrentSymbol>;
pub(crate) type CurrentSymbolMap<V> = ArenaMap<CurrentSymbol, V>;
pub(crate) type CurrentSymbolOrderedMap<V> = ArenaOrderedMap<CurrentSymbol, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentSymbolIdx {
    Inherited(InheritedSymbolIdx),
    Current(CurrentSymbolIdx),
}

impl From<InheritedSymbolIdx> for ParentSymbolIdx {
    fn from(v: InheritedSymbolIdx) -> Self {
        Self::Inherited(v)
    }
}

impl From<CurrentSymbolIdx> for ParentSymbolIdx {
    fn from(v: CurrentSymbolIdx) -> Self {
        Self::Current(v)
    }
}
