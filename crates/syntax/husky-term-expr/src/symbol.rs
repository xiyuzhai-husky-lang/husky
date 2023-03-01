mod context_mut;
mod region;

pub use self::context_mut::*;
pub use self::region::*;

use crate::*;
use idx_arena::map::ArenaMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum TermExprSymbol {
    Entity(EntityPath),
    Inherited(InheritedTermExprSymbolIdx, InheritedTermExprSymbolKind),
    Local(CurrentTermExprSymbolIdx, CurrentTermExprSymbolKind),
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub struct InheritedTermExprSymbol {
    parent_symbol_idx: ParentSymbolIdx,
    kind: InheritedTermExprSymbolKind,
}

impl InheritedTermExprSymbol {
    pub fn kind(&self) -> InheritedTermExprSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn ident(&self) -> Option<Identifier> {
        match self.kind {
            InheritedTermExprSymbolKind::ImplicitParameter(kind) => match kind {
                InheritedImplicitParameterSymbol::Lifetime { label } => None,
                InheritedImplicitParameterSymbol::Type { ident } => Some(ident),
            },
            InheritedTermExprSymbolKind::RegularParameter { ident } => Some(ident),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum InheritedTermExprSymbolKind {
    ImplicitParameter(InheritedImplicitParameterSymbol),
    RegularParameter { ident: Identifier },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedImplicitParameterSymbol {
    Lifetime { label: Label },
    Type { ident: Identifier },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub struct CurrentTermExprSymbol {
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    variant: CurrentTermExprSymbolVariant,
}

impl CurrentTermExprSymbol {
    pub fn new(
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        variant: CurrentTermExprSymbolVariant,
    ) -> Self {
        Self {
            access_start,
            access_end,
            variant,
        }
    }

    pub fn kind(&self) -> CurrentTermExprSymbolKind {
        self.variant.kind()
    }

    pub fn variant(&self) -> &CurrentTermExprSymbolVariant {
        &self.variant
    }

    pub fn ident(&self) -> Option<Identifier> {
        match self.variant {
            CurrentTermExprSymbolVariant::ImplicitParameter {
                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type { ident_token },
            } => Some(ident_token.ident()),
            CurrentTermExprSymbolVariant::RegularParameter { ident, .. }
            | CurrentTermExprSymbolVariant::LetVariable { ident, .. } => Some(ident),
            CurrentTermExprSymbolVariant::ImplicitParameter {
                implicit_parameter_variant: CurrentImplicitParameterSymbol::Lifetime { .. },
            } => None,
        }
    }

    pub fn label(&self) -> Option<Label> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum CurrentTermExprSymbolKind {
    ImplicitParameter {
        implicit_parameter_kind: CurrentImplicitParameterSymbolKind,
    },
    Parameter {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    LetVariable {
        pattern_symbol_idx: PatternSymbolIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum CurrentImplicitParameterSymbolKind {
    Type { ident_token: IdentifierToken },
    Lifetime { label_token: LifetimeLabelToken },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
pub enum CurrentTermExprSymbolVariant {
    ImplicitParameter {
        implicit_parameter_variant: CurrentImplicitParameterSymbol,
    },
    RegularParameter {
        ident: Identifier,
        pattern_symbol_idx: PatternSymbolIdx,
    },
    LetVariable {
        ident: Identifier,
        pattern_symbol_idx: PatternSymbolIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityPathDb)]
#[non_exhaustive]
pub enum CurrentImplicitParameterSymbol {
    Lifetime { label_token: LifetimeLabelToken },
    Type { ident_token: IdentifierToken },
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
        }
    }
}

impl CurrentTermExprSymbolVariant {
    pub fn kind(&self) -> CurrentTermExprSymbolKind {
        match self {
            CurrentTermExprSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => CurrentTermExprSymbolKind::ImplicitParameter {
                implicit_parameter_kind: implicit_parameter_variant.kind(),
            },
            CurrentTermExprSymbolVariant::RegularParameter {
                pattern_symbol_idx, ..
            } => CurrentTermExprSymbolKind::Parameter {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
            CurrentTermExprSymbolVariant::LetVariable {
                pattern_symbol_idx, ..
            } => CurrentTermExprSymbolKind::LetVariable {
                pattern_symbol_idx: *pattern_symbol_idx,
            },
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
        }
    }
}

pub type InheritedTermExprSymbolArena = Arena<InheritedTermExprSymbol>;
pub type InheritedTermExprSymbolIdx = ArenaIdx<InheritedTermExprSymbol>;
pub type InheritedTermExprSymbolIdxRange = ArenaIdxRange<InheritedTermExprSymbol>;
pub type InheritedTermExprSymbolMap<V> = ArenaMap<InheritedTermExprSymbol, V>;

pub type CurrentTermExprSymbolArena = Arena<CurrentTermExprSymbol>;
pub type CurrentTermExprSymbolIdx = ArenaIdx<CurrentTermExprSymbol>;
pub type CurrentTermExprSymbolIdxRange = ArenaIdxRange<CurrentTermExprSymbol>;
pub type CurrentTermExprSymbolMap<V> = ArenaMap<CurrentTermExprSymbol, V>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentSymbolIdx {
    Inherited(InheritedTermExprSymbolIdx),
    Current(CurrentTermExprSymbolIdx),
}

impl From<InheritedTermExprSymbolIdx> for ParentSymbolIdx {
    fn from(v: InheritedTermExprSymbolIdx) -> Self {
        Self::Inherited(v)
    }
}

impl From<CurrentTermExprSymbolIdx> for ParentSymbolIdx {
    fn from(v: CurrentTermExprSymbolIdx) -> Self {
        Self::Current(v)
    }
}
