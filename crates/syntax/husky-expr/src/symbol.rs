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
    Local(LocalSymbolIdx, LocalSymbolKind),
}

#[derive(Debug, PartialEq, Eq)]
pub struct InheritedSymbol {
    ident: Identifier,
    kind: InheritedSymbolKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InheritedSymbolKind {
    Parameter {
        original_local_symbol_idx: LocalSymbolIdx,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LocalSymbol {
    ident: Identifier,
    access_start: TokenIdx,
    /// this is none only for lambda variable
    access_end: Option<TokenIdxRangeEnd>,
    kind: LocalSymbolKind,
}

impl LocalSymbol {
    pub fn new(
        ident: Identifier,
        access_start: TokenIdx,
        access_end: Option<TokenIdxRangeEnd>,
        variant: LocalSymbolKind,
    ) -> Self {
        Self {
            ident,
            access_start,
            access_end,
            kind: variant,
        }
    }

    pub fn kind(&self) -> LocalSymbolKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocalSymbolKind {
    Parameter { pattern_symbol: PatternSymbolIdx },
    LetVariable { pattern_symbol: PatternSymbolIdx },
}

pub type InheritedSymbolArena = Arena<InheritedSymbol>;
pub type InheritedSymbolIdx = ArenaIdx<InheritedSymbol>;
pub type InheritedSymbolIdxRange = ArenaIdxRange<InheritedSymbol>;

pub type LocalSymbolArena = Arena<LocalSymbol>;
pub type LocalSymbolIdx = ArenaIdx<LocalSymbol>;
pub type LocalSymbolIdxRange = ArenaIdxRange<LocalSymbol>;
