mod context_mut;
mod region;

pub use context_mut::*;
pub use region::*;

use crate::*;
use husky_entity_tree::{CrateSymbolContext, ModuleSymbolContext, PreludeResult};
use husky_term::Term;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum Symbol {
    Entity(EntityPath),
    Inherited(InheritedSymbolIdx, InheritedSymbolKind),
    Local(CurrentSymbolIdx, CurrentSymbolKind),
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct InheritedSymbol {
    ident: Identifier,
    parent_symbol_idx: ParentSymbolIdx,
    kind: InheritedSymbolKind,
}

impl InheritedSymbol {
    pub fn kind(&self) -> InheritedSymbolKind {
        self.kind
    }

    pub fn parent_symbol_idx(&self) -> ParentSymbolIdx {
        self.parent_symbol_idx
    }

    pub fn ident(&self) -> Identifier {
        self.ident
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum InheritedSymbolKind {
    ImplicitParameter,
    RegularParameter,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct CurrentSymbol {
    ident: Identifier,
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    variant: CurrentSymbolVariant,
}

impl CurrentSymbol {
    pub fn new(
        ident: Identifier,
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        variant: CurrentSymbolVariant,
    ) -> Self {
        Self {
            ident,
            access_start,
            access_end,
            variant,
        }
    }

    pub fn kind(&self) -> CurrentSymbolKind {
        self.variant.kind()
    }

    pub fn variant(&self) -> &CurrentSymbolVariant {
        &self.variant
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum CurrentSymbolKind {
    ImplicitParameter {
        implicit_parameter_kind: ImplicitParameterKind,
    },
    Parameter {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    LetVariable {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    FrameVariable(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum ImplicitParameterKind {
    Type { ident_token: IdentifierToken },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum CurrentSymbolVariant {
    ImplicitParameter {
        implicit_parameter_variant: ImplicitParameterVariant,
    },
    RegularParameter {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    LetVariable {
        pattern_symbol_idx: PatternSymbolIdx,
    },
    FrameVariable(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprDb)]
#[non_exhaustive]
pub enum ImplicitParameterVariant {
    Lifetime,
    Type { ident_token: IdentifierToken },
}

impl CurrentSymbolVariant {
    pub fn kind(&self) -> CurrentSymbolKind {
        match self {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => CurrentSymbolKind::ImplicitParameter {
                implicit_parameter_kind: implicit_parameter_variant.kind(),
            },
            CurrentSymbolVariant::RegularParameter { pattern_symbol_idx } => {
                CurrentSymbolKind::Parameter {
                    pattern_symbol_idx: *pattern_symbol_idx,
                }
            }
            CurrentSymbolVariant::LetVariable { pattern_symbol_idx } => {
                CurrentSymbolKind::LetVariable {
                    pattern_symbol_idx: *pattern_symbol_idx,
                }
            }
            CurrentSymbolVariant::FrameVariable(variable_idx) => {
                CurrentSymbolKind::FrameVariable(*variable_idx)
            }
        }
    }
}

impl ImplicitParameterVariant {
    fn kind(&self) -> ImplicitParameterKind {
        match self {
            ImplicitParameterVariant::Type { ident_token } => ImplicitParameterKind::Type {
                ident_token: *ident_token,
            },
            ImplicitParameterVariant::Lifetime => todo!(),
        }
    }
}

pub type InheritedSymbolArena = Arena<InheritedSymbol>;
pub type InheritedSymbolIdx = ArenaIdx<InheritedSymbol>;
pub type InheritedSymbolIdxRange = ArenaIdxRange<InheritedSymbol>;
pub type InheritedSymbolMap<V> = ArenaMap<InheritedSymbol, V>;

pub type CurrentSymbolArena = Arena<CurrentSymbol>;
pub type CurrentSymbolIdx = ArenaIdx<CurrentSymbol>;
pub type CurrentSymbolIdxRange = ArenaIdxRange<CurrentSymbol>;
pub type CurrentSymbolMap<V> = ArenaMap<CurrentSymbol, V>;

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
