use idx_arena::ordered_map::ArenaOrderedMap;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub enum PatternSymbol {
    Atom(PatternExprIdx),
}

pub type PatternSymbolArena = Arena<PatternSymbol>;
pub type PatternSymbolIdx = ArenaIdx<PatternSymbol>;
pub type PatternSymbolMap<V> = ArenaMap<PatternSymbol, V>;
pub type PatternSymbolOrderedMap<V> = ArenaOrderedMap<PatternSymbol, V>;
