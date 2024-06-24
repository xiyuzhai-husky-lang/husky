use super::*;

/// not cloneable intentionally
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolOrderedMap<V> {
    inherited_variable_map: InheritedVariableOrderedMap<V>,
    current_variable_map: CurrentVariableOrderedMap<V>,
}

impl<V> SymbolOrderedMap<V> {
    pub fn new(parent: Option<&Self>) -> Self
    where
        V: Clone,
    {
        Self {
            inherited_variable_map: match parent {
                Some(parent) => {
                    let mut inherited_variable_map = parent.inherited_variable_map.clone();
                    for v in parent.current_variable_map.iter() {
                        unsafe { inherited_variable_map.insert_next_unchecked(v.clone()) }
                    }
                    inherited_variable_map
                }
                None => Default::default(),
            },
            current_variable_map: Default::default(),
        }
    }

    pub fn insert_next(&mut self, idx: CurrentVariableIdx, v: V) {
        self.current_variable_map.insert_next(idx, v)
    }

    pub fn inherited_variable_map(&self) -> &InheritedVariableOrderedMap<V> {
        &self.inherited_variable_map
    }

    pub fn current_variable_map(&self) -> &CurrentVariableOrderedMap<V> {
        &self.current_variable_map
    }
}

impl<V> std::ops::Index<InheritedVariableIdx> for SymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, index: InheritedVariableIdx) -> &Self::Output {
        &self.inherited_variable_map[index]
    }
}

impl<V> std::ops::Index<CurrentVariableIdx> for SymbolOrderedMap<V> {
    type Output = V;

    fn index(&self, index: CurrentVariableIdx) -> &Self::Output {
        &self.current_variable_map[index]
    }
}
