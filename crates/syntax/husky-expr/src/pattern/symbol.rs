use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum PatternSymbol {
    Atom(PatternExprIdx),
}

pub type PatternSymbolArena = Arena<PatternSymbol>;
pub type PatternSymbolIdx = ArenaIdx<PatternSymbol>;
