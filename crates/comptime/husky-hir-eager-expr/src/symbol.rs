use crate::*;

pub struct InheritedHirEagerSymbol {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InheritedHirEagerSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct CurrentHirEagerSymbol {
    modifier: EphemSymbolModifier,
    variant: CurrentHirSymbolVariant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentHirSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentHirSymbolVariant {}

pub type InheritedHirSymbolArena = Arena<InheritedHirEagerSymbol>;
pub type InheritedHirSymbolIdx = ArenaIdx<InheritedHirEagerSymbol>;
pub type InheritedHirSymbolIdxRange = ArenaIdxRange<InheritedHirEagerSymbol>;
pub(crate) type InheritedHirSymbolMap<V> = ArenaMap<InheritedHirEagerSymbol, V>;
pub(crate) type InheritedHirSymbolOrderedMap<V> = ArenaOrderedMap<InheritedHirEagerSymbol, V>;

pub type CurrentHirEagerSymbolArena = Arena<CurrentHirEagerSymbol>;
pub type CurrentHirEagerSymbolIdx = ArenaIdx<CurrentHirEagerSymbol>;
pub type CurrentHirEagerSymbolIdxRange = ArenaIdxRange<CurrentHirEagerSymbol>;
pub(crate) type CurrentHirEagerSymbolMap<V> = ArenaMap<CurrentHirEagerSymbol, V>;
pub(crate) type CurrentHirEagerSymbolOrderedMap<V> = ArenaOrderedMap<CurrentHirEagerSymbol, V>;
