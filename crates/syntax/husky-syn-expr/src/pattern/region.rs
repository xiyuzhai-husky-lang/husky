use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct SynPatternRegion {
    pattern_arena: SynPatternArena,
    /// the contract of pattern expressions are computed when they are created
    pattern_contracts: SynPatternOrderedMap<Contract>,
    pattern_variable_arena: SynPatternSymbolArena,
    pattern_variable_maps: SynPatternOrderedMap<IdentPairMap<PatternVariableIdx>>,
    pattern_variable_modifiers: SynPatternSymbolOrderedMap<VariableModifier>,
}

impl SynPatternRegion {
    pub fn alloc_one_pattern(&mut self, expr: SynPatternData) -> SynPatternIdx {
        // order matters
        let contract = expr.contract();
        let idx = self.pattern_arena.alloc_one(expr);
        let symbols = self.calc_symbols(idx);
        assert_eq!(idx.index(), self.pattern_variable_maps.len());
        self.pattern_variable_maps.insert_next(idx, symbols);
        self.pattern_contracts.insert_next(idx, contract);
        idx
    }

    // expr must be allocated already
    fn calc_symbols(&mut self, pattern_idx: SynPatternIdx) -> IdentPairMap<PatternVariableIdx> {
        let symbols: IdentPairMap<PatternVariableIdx> = match self.pattern_arena[pattern_idx] {
            SynPatternData::Literal { .. } => Default::default(),
            SynPatternData::Ident { ident_token, .. } => IdentPairMap::new_one_element_map((
                ident_token.ident(),
                self.alloc_new_symbol(PatternVariable::Atom(pattern_idx)),
            )),
            SynPatternData::UnitTypeVariant { .. } => Default::default(),
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            // ad hoc
            SynPatternData::TupleTypeVariant { .. } => Default::default(),
            SynPatternData::Props { .. } => todo!(),
            SynPatternData::OneOf { ref options } => {
                debug_assert!(options.elements().len() > 1);
                let symbols = self.pattern_variable_maps[options.elements()[0].syn_pattern()].clone();
                for option in &options.elements()[1..] {
                    let option_symbols = &self.pattern_variable_maps[option.syn_pattern()];
                    if option_symbols != &symbols {
                        todo!()
                    }
                }
                symbols
            }
            SynPatternData::Binding { .. } => todo!(),
            SynPatternData::Range { .. } => todo!(),
        };
        symbols
    }

    fn alloc_new_symbol(&mut self, symbol: PatternVariable) -> PatternVariableIdx {
        let modifier = symbol.pattern_variable_modifier(&self.pattern_arena);
        let idx = self.pattern_variable_arena.alloc_one(symbol);
        self.pattern_variable_modifiers.insert_next(idx, modifier);
        idx
    }

    pub fn patterns<'a>(
        &'a self,
    ) -> impl Iterator<Item = (SynPatternIdx, &'a SynPatternData)> + 'a {
        self.pattern_arena.indexed_iter()
    }

    pub fn pattern_variables(
        &self,
        syn_pattern_idx: SynPatternIdx,
    ) -> &[(Ident, PatternVariableIdx)] {
        &self.pattern_variable_maps[syn_pattern_idx]
    }

    pub fn pattern_arena(&self) -> &SynPatternArena {
        &self.pattern_arena
    }

    pub fn pattern_variable_arena(&self) -> &SynPatternSymbolArena {
        &self.pattern_variable_arena
    }
}

impl std::ops::Index<SynPatternIdx> for SynPatternRegion {
    type Output = SynPatternData;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
        &self.pattern_arena[index]
    }
}

impl std::ops::Index<PatternVariableIdx> for SynPatternRegion {
    type Output = PatternVariable;

    fn index(&self, index: PatternVariableIdx) -> &Self::Output {
        &self.pattern_variable_arena[index]
    }
}

impl std::ops::Index<&PatternVariableIdx> for SynPatternRegion {
    type Output = PatternVariable;

    fn index(&self, index: &PatternVariableIdx) -> &Self::Output {
        &self.pattern_variable_arena[index]
    }
}

impl SynExprRegionData {
    pub fn pattern_contract(&self, syn_pattern: SynPatternIdx) -> Contract {
        self.pattern_region().pattern_contract(syn_pattern)
    }

    pub fn pattern_variable_modifier(&self, pattern_variable: PatternVariableIdx) -> VariableModifier {
        self.pattern_region()
            .pattern_variable_modifier(pattern_variable)
    }
}

impl SynPatternRegion {
    fn pattern_contract(&self, pattern_idx: SynPatternIdx) -> Contract {
        self.pattern_contracts[pattern_idx]
    }

    pub fn pattern_variable_modifier(
        &self,
        pattern_variable_idx: PatternVariableIdx,
    ) -> VariableModifier {
        self.pattern_variable_modifiers[pattern_variable_idx]
    }
}
