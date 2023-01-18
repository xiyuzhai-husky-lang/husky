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
    Parameter {
        original_current_symbol_idx: CurrentSymbolIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CurrentSymbol {
    ident: Identifier,
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    kind: CurrentSymbolKind,
}

impl CurrentSymbol {
    pub fn new(
        ident: Identifier,
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        kind: CurrentSymbolKind,
    ) -> Self {
        Self {
            ident,
            access_start,
            access_end,
            kind,
        }
    }

    pub fn kind(&self) -> CurrentSymbolKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentSymbolKind {
    Parameter { pattern_symbol: PatternSymbolIdx },
    LetVariable { pattern_symbol: PatternSymbolIdx },
    FrameVariable(ExprIdx),
}

pub type InheritedSymbolArena = Arena<InheritedSymbol>;
pub type InheritedSymbolIdx = ArenaIdx<InheritedSymbol>;
pub type InheritedSymbolIdxRange = ArenaIdxRange<InheritedSymbol>;

pub type CurrentSymbolArena = Arena<CurrentSymbol>;
pub type CurrentSymbolIdx = ArenaIdx<CurrentSymbol>;
pub type CurrentSymbolIdxRange = ArenaIdxRange<CurrentSymbol>;
