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

    pub fn ident(&self) -> Option<Identifier> {
        match self.kind {
            InheritedSymbolKind::ImplicitParameter(kind) => match kind {
                InheritedImplicitParameterSymbol::Lifetime { label } => None,
                InheritedImplicitParameterSymbol::Type { ident } => Some(ident),
            },
            InheritedSymbolKind::RegularParameter { ident } => Some(ident),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum InheritedSymbolKind {
    ImplicitParameter(InheritedImplicitParameterSymbol),
    RegularParameter { ident: Identifier },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedImplicitParameterSymbol {
    Lifetime { label: Label },
    Type { ident: Identifier },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct CurrentSymbol {
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    variant: CurrentSymbolVariant,
}

impl CurrentSymbol {
    pub fn new(
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        variant: CurrentSymbolVariant,
    ) -> Self {
        Self {
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

    pub fn ident(&self) -> Option<Identifier> {
        match self.variant {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type { ident_token },
            } => Some(ident_token.ident()),
            CurrentSymbolVariant::RegularParameter { ident, .. }
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
pub enum CurrentImplicitParameterSymbolKind {
    Type { ident_token: IdentifierToken },
    Lifetime { label_token: LifetimeLabelToken },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum CurrentSymbolVariant {
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
    FrameVariable {
        ident: Identifier,
        expr_idx: ExprIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprDb)]
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

impl CurrentSymbolVariant {
    pub fn kind(&self) -> CurrentSymbolKind {
        match self {
            CurrentSymbolVariant::ImplicitParameter {
                implicit_parameter_variant,
            } => CurrentSymbolKind::ImplicitParameter {
                implicit_parameter_kind: implicit_parameter_variant.kind(),
            },
            CurrentSymbolVariant::RegularParameter {
                pattern_symbol_idx, ..
            } => CurrentSymbolKind::Parameter {
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
