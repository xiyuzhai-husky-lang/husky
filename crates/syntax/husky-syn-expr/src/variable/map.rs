use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolMap<V> {
    inherited_variable_map: InheritedVariableMap<V>,
    current_variable_map: CurrentVariableMap<V>,
}

impl<V> SymbolMap<V> {
    pub fn new(parent: Option<&Self>, region: &VariableRegionData) -> Self
    where
        V: Clone,
    {
        let inherited_variable_arena = region.inherited_variable_arena();
        let current_variable_arena = region.current_variable_arena();
        Self {
            inherited_variable_map: match parent {
                Some(parent) => {
                    let mut inherited_variable_map = parent
                        .inherited_variable_map
                        .clone_for_extended(inherited_variable_arena);
                    let base = parent.inherited_variable_map.len();
                    for (parent_idx, v) in parent.current_variable_map.key_value_iter() {
                        unsafe {
                            let idx = ArenaIdx::from_raw(base + parent_idx.index());
                            inherited_variable_map.insert_new(idx, v.clone())
                        }
                    }
                    inherited_variable_map
                }
                None => InheritedVariableMap::new(inherited_variable_arena),
            },
            current_variable_map: CurrentVariableMap::new(current_variable_arena),
        }
    }

    pub fn insert_new(&mut self, idx: CurrentVariableIdx, v: V) {
        self.current_variable_map.insert_new(idx, v)
    }

    pub fn inherited_variable_map(&self) -> &InheritedVariableMap<V> {
        &self.inherited_variable_map
    }

    pub fn current_variable_map(&self) -> &CurrentVariableMap<V> {
        &self.current_variable_map
    }

    pub fn current_variables(&self) -> &CurrentVariableMap<V> {
        &self.current_variable_map
    }

    pub fn inherited_variable_key_values(
        &self,
    ) -> impl Iterator<Item = (InheritedVariableIdx, &V)> {
        self.inherited_variable_map.key_value_iter()
    }

    pub fn current_variable_key_values(&self) -> impl Iterator<Item = (CurrentVariableIdx, &V)> {
        self.current_variable_map.key_value_iter()
    }
}

impl<V> std::ops::Index<InheritedVariableIdx> for SymbolMap<V> {
    type Output = V;

    fn index(&self, index: InheritedVariableIdx) -> &Self::Output {
        &self.inherited_variable_map[index]
    }
}

impl<V> std::ops::Index<CurrentVariableIdx> for SymbolMap<V> {
    type Output = V;

    fn index(&self, index: CurrentVariableIdx) -> &Self::Output {
        &self.current_variable_map[index]
    }
}
