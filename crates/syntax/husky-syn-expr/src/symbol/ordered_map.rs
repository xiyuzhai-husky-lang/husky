use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolOrderedMap<V> {
    inherited_symbol_map: InheritedSynSymbolOrderedMap<V>,
    current_symbol_map: CurrentSynSymbolOrderedMap<V>,
}

impl<V> SymbolOrderedMap<V> {
    pub fn new(parent: Option<&Self>) -> Self
    where
        V: Clone,
    {
        Self {
            inherited_symbol_map: match parent {
                Some(parent) => {
                    let mut inherited_symbol_map = parent.inherited_symbol_map.clone();
                    for v in parent.current_symbol_map.iter() {
                        unsafe { inherited_symbol_map.insert_next_unchecked(v.clone()) }
                    }
                    inherited_symbol_map
                }
                None => Default::default(),
            },
            current_symbol_map: Default::default(),
        }
    }

    pub fn insert_next(&mut self, idx: CurrentSynSymbolIdx, v: V) {
        self.current_symbol_map.insert_next(idx, v)
    }

    pub fn inherited_symbol_map(&self) -> &InheritedSynSymbolOrderedMap<V> {
        &self.inherited_symbol_map
    }

    pub fn current_symbol_map(&self) -> &CurrentSynSymbolOrderedMap<V> {
        &self.current_symbol_map
    }
}

impl<V> std::ops::Index<InheritedSynSymbolIdx> for SymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, index: InheritedSynSymbolIdx) -> &Self::Output {
        &self.inherited_symbol_map[index]
    }
}

impl<V> std::ops::Index<CurrentSynSymbolIdx> for SymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, index: CurrentSynSymbolIdx) -> &Self::Output {
        &self.current_symbol_map[index]
    }
}
