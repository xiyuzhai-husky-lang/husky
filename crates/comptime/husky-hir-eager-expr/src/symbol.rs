use crate::*;

pub struct InheritedHirEagerSymbol {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InheritedHirEagerSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = HirEagerExprDb)]
pub struct CurrentHirEagerSymbol {
    modifier: EphemSymbolModifier,
    variant: CurrentHirEagerSymbolVariant,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentHirEagerSymbolKind {}

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentHirEagerSymbolVariant {}

pub type InheritedHirEagerSymbolArena = Arena<InheritedHirEagerSymbol>;
pub type InheritedHirEagerSymbolIdx = ArenaIdx<InheritedHirEagerSymbol>;
pub type InheritedHirEagerSymbolIdxRange = ArenaIdxRange<InheritedHirEagerSymbol>;
pub(crate) type InheritedHirEagerSymbolMap<V> = ArenaMap<InheritedHirEagerSymbol, V>;
pub(crate) type InheritedHirEagerSymbolOrderedMap<V> = ArenaOrderedMap<InheritedHirEagerSymbol, V>;

pub type CurrentHirEagerSymbolArena = Arena<CurrentHirEagerSymbol>;
pub type CurrentHirEagerSymbolIdx = ArenaIdx<CurrentHirEagerSymbol>;
pub type CurrentHirEagerSymbolIdxRange = ArenaIdxRange<CurrentHirEagerSymbol>;
pub(crate) type CurrentHirEagerSymbolMap<V> = ArenaMap<CurrentHirEagerSymbol, V>;
pub(crate) type CurrentHirEagerSymbolOrderedMap<V> = ArenaOrderedMap<CurrentHirEagerSymbol, V>;
