mod context_mut;
mod sheet;

pub use context_mut::*;
pub use sheet::*;

use crate::*;
use husky_entity_tree::{CrateSymbolContext, ModuleSymbolContext, PreludeResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Entity(EntityPath),
    Inherited(InheritedSymbolIdx, InheritedSymbolKind),
    Local(CurrentSymbolIdx, CurrentSymbolKind),
}

#[derive(Debug, PartialEq, Eq)]
pub struct InheritedSymbol {
    ident: Identifier,
    kind: InheritedSymbolKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedSymbolKind {
    Parameter,
    ImplicitParameter,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
pub enum CurrentSymbolKind {
    ImplicitParameter {
        implicit_parameter_kind: ImplicitParameterKind,
    },
    Parameter {
        pattern_symbol: PatternSymbolIdx,
    },
    LetVariable {
        pattern_symbol: PatternSymbolIdx,
    },
    FrameVariable(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitParameterKind {
    Type { ident_token: IdentifierToken },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CurrentSymbolVariant {
    ImplicitParameter {
        implicit_parameter_variant: ImplicitParameterVariant,
    },
    Parameter {
        pattern_symbol: PatternSymbolIdx,
    },
    LetVariable {
        pattern_symbol: PatternSymbolIdx,
    },
    FrameVariable(ExprIdx),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImplicitParameterVariant {
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
            CurrentSymbolVariant::Parameter { pattern_symbol } => CurrentSymbolKind::Parameter {
                pattern_symbol: *pattern_symbol,
            },
            CurrentSymbolVariant::LetVariable { pattern_symbol } => {
                CurrentSymbolKind::LetVariable {
                    pattern_symbol: *pattern_symbol,
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
        }
    }
}

pub type InheritedSymbolArena = Arena<InheritedSymbol>;
pub type InheritedSymbolIdx = ArenaIdx<InheritedSymbol>;
pub type InheritedSymbolIdxRange = ArenaIdxRange<InheritedSymbol>;

pub type CurrentSymbolArena = Arena<CurrentSymbol>;
pub type CurrentSymbolIdx = ArenaIdx<CurrentSymbol>;
pub type CurrentSymbolIdxRange = ArenaIdxRange<CurrentSymbol>;
