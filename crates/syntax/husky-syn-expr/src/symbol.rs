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
use husky_entity_syn_tree::{CratePrelude, ModuleSymbolContext, PreludeResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum Symbol {
    PrincipalEntity(PrincipalEntityPath),
    Inherited(InheritedSynSymbolIdx, InheritedSynSymbolKind),
    Local(CurrentSynSymbolIdx, CurrentSynSymbolKind),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum ImplicitParameterSymbol {
    Lifetime { label_token: LifetimeLabelToken },
    Type { ident_token: IdentToken },
    Const {},
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct InheritedSynSymbol {
    parent_symbol_idx: ParentSynSymbolIdx,
    modifier: SymbolModifier,
    kind: InheritedSynSymbolKind,
}

impl InheritedSynSymbol {
    pub fn kind(&self) -> InheritedSynSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSynSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.kind {
            InheritedSynSymbolKind::ImplicitParameter(kind) => match kind {
                InheritedImplicitParameterSynSymbol::Lifetime { label } => None,
                InheritedImplicitParameterSynSymbol::Type { ident }
                | InheritedImplicitParameterSynSymbol::Constant { ident } => Some(ident),
            },
            InheritedSynSymbolKind::ExplicitParameter { ident } => Some(ident),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum InheritedSynSymbolKind {
    ImplicitParameter(InheritedImplicitParameterSynSymbol),
    ExplicitParameter { ident: Ident },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum InheritedImplicitParameterSynSymbol {
    Lifetime { label: Label },
    Type { ident: Ident },
    Constant { ident: Ident },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct CurrentSynSymbol {
    modifier: SymbolModifier,
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    variant: CurrentSynSymbolVariant,
}

impl CurrentSynSymbol {
    pub fn new(
        pattern_expr_region: &PatternSynExprRegion,
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        variant: CurrentSynSymbolVariant,
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

    pub fn kind(&self) -> CurrentSynSymbolKind {
        self.variant.kind()
    }

    pub fn variant(&self) -> &CurrentSynSymbolVariant {
        &self.variant
    }

    pub fn ident(&self) -> Option<Ident> {
        match self.variant {
            CurrentSynSymbolVariant::ImplicitParameter {
                template_parameter_variant:
                    CurrentImplicitParameterSymbol::Type { ident_token }
                    | CurrentImplicitParameterSymbol::Constant { ident_token, .. },
            }
            | CurrentSynSymbolVariant::ExplicitVariadicParameter { ident_token, .. } => {
                Some(ident_token.ident())
            }
            CurrentSynSymbolVariant::ExplicitRegularParameter { ident, .. }
            | CurrentSynSymbolVariant::LetVariable { ident, .. }
            | CurrentSynSymbolVariant::FrameVariable { ident, .. } => Some(ident),
            CurrentSynSymbolVariant::ImplicitParameter {
                template_parameter_variant: CurrentImplicitParameterSymbol::Lifetime { .. },
            } => None,
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum CurrentSynSymbolKind {
    ImplicitParameter {
        template_parameter_kind: CurrentImplicitParameterSynSymbolKind,
    },
    ExplicitRegularParameter {
        pattern_symbol_idx: PatternSynSymbolIdx,
    },
    ExplicitVariadicParameter {
        ident_token: IdentToken,
    },
    LetVariable {
        pattern_symbol_idx: PatternSynSymbolIdx,
    },
    FrameVariable(SynExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum CurrentImplicitParameterSynSymbolKind {
    Type { ident_token: IdentToken },
    Lifetime { label_token: LifetimeLabelToken },
    Constant { ident_token: IdentToken },
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum CurrentSynSymbolVariant {
    ImplicitParameter {
        template_parameter_variant: CurrentImplicitParameterSymbol,
    },
    ExplicitRegularParameter {
        ident: Ident,
        pattern_symbol_idx: PatternSynSymbolIdx,
    },
    ExplicitVariadicParameter {
        symbol_modifier_keyword_group: Option<SymbolModifierKeywordGroup>,
        ident_token: IdentToken,
    },
    LetVariable {
        ident: Ident,
        pattern_symbol_idx: PatternSynSymbolIdx,
    },
    FrameVariable {
        ident: Ident,
        expr_idx: SynExprIdx,
    },
}

impl CurrentSynSymbolVariant {
    fn modifier(&self, pattern_expr_region: &PatternSynExprRegion) -> SymbolModifier {
        match self {
            CurrentSynSymbolVariant::ImplicitParameter {
                template_parameter_variant,
            } => SymbolModifier::Const,
            CurrentSynSymbolVariant::ExplicitRegularParameter {
                pattern_symbol_idx, ..
            }
            | CurrentSynSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => pattern_expr_region.pattern_symbol_modifier(*pattern_symbol_idx),
            CurrentSynSymbolVariant::ExplicitVariadicParameter {
                symbol_modifier_keyword_group,
                ..
            } => SymbolModifier::new(*symbol_modifier_keyword_group),
            CurrentSynSymbolVariant::FrameVariable { ident, expr_idx } => SymbolModifier::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
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
        ty_expr_idx: SynExprIdx,
    },
}

impl CurrentImplicitParameterSymbol {
    fn bequeath(&self) -> InheritedImplicitParameterSynSymbol {
        match self {
            CurrentImplicitParameterSymbol::Lifetime { label_token } => {
                InheritedImplicitParameterSynSymbol::Lifetime {
                    label: label_token.label(),
                }
            }
            CurrentImplicitParameterSymbol::Type { ident_token } => {
                InheritedImplicitParameterSynSymbol::Type {
                    ident: ident_token.ident(),
                }
            }
            CurrentImplicitParameterSymbol::Constant {
                ident_token,
                ty_expr_idx,
            } => InheritedImplicitParameterSynSymbol::Constant {
                ident: ident_token.ident(),
            },
        }
    }
}

impl CurrentSynSymbolVariant {
    pub fn kind(&self) -> CurrentSynSymbolKind {
        match self {
            CurrentSynSymbolVariant::ImplicitParameter {
                template_parameter_variant,
            } => CurrentSynSymbolKind::ImplicitParameter {
                template_parameter_kind: template_parameter_variant.kind(),
            },
            CurrentSynSymbolVariant::ExplicitRegularParameter {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::ExplicitRegularParameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSynSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => CurrentSynSymbolKind::LetVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentSynSymbolVariant::FrameVariable { expr_idx, .. } => {
                CurrentSynSymbolKind::FrameVariable(*expr_idx)
            }
            CurrentSynSymbolVariant::ExplicitVariadicParameter { ident_token, .. } => {
                CurrentSynSymbolKind::ExplicitVariadicParameter {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

impl CurrentImplicitParameterSymbol {
    fn kind(&self) -> CurrentImplicitParameterSynSymbolKind {
        match self {
            CurrentImplicitParameterSymbol::Type { ident_token } => {
                CurrentImplicitParameterSynSymbolKind::Type {
                    ident_token: *ident_token,
                }
            }
            CurrentImplicitParameterSymbol::Lifetime { label_token } => {
                CurrentImplicitParameterSynSymbolKind::Lifetime {
                    label_token: *label_token,
                }
            }
            CurrentImplicitParameterSymbol::Constant { ident_token, .. } => {
                CurrentImplicitParameterSynSymbolKind::Constant {
                    ident_token: *ident_token,
                }
            }
        }
    }
}

pub type InheritedSynSymbolArena = Arena<InheritedSynSymbol>;
pub type InheritedSynSymbolIdx = ArenaIdx<InheritedSynSymbol>;
pub type InheritedSynSymbolIdxRange = ArenaIdxRange<InheritedSynSymbol>;
pub(crate) type InheritedSynSymbolMap<V> = ArenaMap<InheritedSynSymbol, V>;
pub(crate) type InheritedSynSymbolOrderedMap<V> = ArenaOrderedMap<InheritedSynSymbol, V>;

pub type CurrentSynSymbolArena = Arena<CurrentSynSymbol>;
pub type CurrentSynSymbolIdx = ArenaIdx<CurrentSynSymbol>;
pub type CurrentSynSymbolIdxRange = ArenaIdxRange<CurrentSynSymbol>;
pub(crate) type CurrentSynSymbolMap<V> = ArenaMap<CurrentSynSymbol, V>;
pub(crate) type CurrentSynSymbolOrderedMap<V> = ArenaOrderedMap<CurrentSynSymbol, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentSynSymbolIdx {
    Inherited(InheritedSynSymbolIdx),
    Current(CurrentSynSymbolIdx),
}

impl From<InheritedSynSymbolIdx> for ParentSynSymbolIdx {
    fn from(v: InheritedSynSymbolIdx) -> Self {
        Self::Inherited(v)
    }
}

impl From<CurrentSynSymbolIdx> for ParentSynSymbolIdx {
    fn from(v: CurrentSynSymbolIdx) -> Self {
        Self::Current(v)
    }
}
