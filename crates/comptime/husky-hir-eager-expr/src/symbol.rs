use crate::*;

pub struct InheritedHirSymbol {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InheritedHirSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = HirEagerExprDb)]
pub struct CurrentHirSymbol {
    modifier: SymbolModifier,
    variant: CurrentHirSymbolVariant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentHirSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentHirSymbolVariant {}

pub type InheritedHirSymbolArena = Arena<InheritedHirSymbol>;
pub type InheritedHirSymbolIdx = ArenaIdx<InheritedHirSymbol>;
pub type InheritedHirSymbolIdxRange = ArenaIdxRange<InheritedHirSymbol>;
pub(crate) type InheritedHirSymbolMap<V> = ArenaMap<InheritedHirSymbol, V>;
pub(crate) type InheritedHirSymbolOrderedMap<V> = ArenaOrderedMap<InheritedHirSymbol, V>;

pub type CurrentHirSymbolArena = Arena<CurrentHirSymbol>;
pub type CurrentHirSymbolIdx = ArenaIdx<CurrentHirSymbol>;
pub type CurrentHirSymbolIdxRange = ArenaIdxRange<CurrentHirSymbol>;
pub(crate) type CurrentHirSymbolMap<V> = ArenaMap<CurrentHirSymbol, V>;
pub(crate) type CurrentHirSymbolOrderedMap<V> = ArenaOrderedMap<CurrentHirSymbol, V>;
