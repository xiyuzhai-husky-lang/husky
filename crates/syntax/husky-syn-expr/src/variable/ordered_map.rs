use super::*;

/// not cloneable intentionally
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolOrderedMap<V> {
    inherited_syn_symbol_map: InheritedSynSymbolOrderedMap<V>,
    current_variable_map: CurrentSynSymbolOrderedMap<V>,
}

impl<V> SymbolOrderedMap<V> {
    pub fn new(parent: Option<&Self>) -> Self
    where
        V: Clone,
    {
        Self {
            inherited_syn_symbol_map: match parent {
                Some(parent) => {
                    let mut inherited_syn_symbol_map = parent.inherited_syn_symbol_map.clone();
                    for v in parent.current_variable_map.iter() {
                        unsafe { inherited_syn_symbol_map.insert_next_unchecked(v.clone()) }
                    }
                    inherited_syn_symbol_map
                }
                None => Default::default(),
            },
            current_variable_map: Default::default(),
        }
    }

    pub fn insert_next(&mut self, idx: CurrentVariableIdx, v: V) {
        self.current_variable_map.insert_next(idx, v)
    }

    pub fn inherited_syn_symbol_map(&self) -> &InheritedSynSymbolOrderedMap<V> {
        &self.inherited_syn_symbol_map
    }

    pub fn current_variable_map(&self) -> &CurrentSynSymbolOrderedMap<V> {
        &self.current_variable_map
    }
}

impl<V> std::ops::Index<InheritedSymbolicVariableIdx> for SymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, index: InheritedSymbolicVariableIdx) -> &Self::Output {
        &self.inherited_syn_symbol_map[index]
    }
}

impl<V> std::ops::Index<CurrentVariableIdx> for SymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, index: CurrentVariableIdx) -> &Self::Output {
        &self.current_variable_map[index]
    }
}
