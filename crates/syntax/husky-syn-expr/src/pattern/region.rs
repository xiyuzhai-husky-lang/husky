use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct SynPatternExprRegion {
    pattern_expr_arena: SynPatternArena,
    /// the contract of pattern expressions are computed when they are created
    pattern_expr_contracts: SynPatternOrderedMap<Contract>,
    pattern_symbol_arena: SynPatternSymbolArena,
    pattern_symbol_maps: SynPatternOrderedMap<IdentPairMap<PatternVariableIdx>>,
    pattern_symbol_modifiers: SynPatternSymbolOrderedMap<VariableModifier>,
}

impl SynPatternExprRegion {
    pub fn alloc_one_pattern_expr(&mut self, expr: SynPatternData) -> SynPatternIdx {
        // order matters
        let contract = expr.contract();
        let idx = self.pattern_expr_arena.alloc_one(expr);
        let symbols = self.calc_symbols(idx);
        assert_eq!(idx.index(), self.pattern_symbol_maps.len());
        self.pattern_symbol_maps.insert_next(idx, symbols);
        self.pattern_expr_contracts.insert_next(idx, contract);
        idx
    }

    // expr must be allocated already
    fn calc_symbols(
        &mut self,
        pattern_expr_idx: SynPatternIdx,
    ) -> IdentPairMap<PatternVariableIdx> {
        let symbols: IdentPairMap<PatternVariableIdx> = match self.pattern_expr_arena
            [pattern_expr_idx]
        {
            SynPatternData::Literal { .. } => Default::default(),
            SynPatternData::Ident { ident_token, .. } => IdentPairMap::new_one_element_map((
                ident_token.ident(),
                self.alloc_new_symbol(PatternVariable::Atom(pattern_expr_idx)),
            )),
            SynPatternData::UnitTypeVariant { .. } => Default::default(),
            SynPatternData::Tuple { .. } => todo!(),
            SynPatternData::TupleStruct { .. } => todo!(),
            // ad hoc
            SynPatternData::TupleTypeVariant { .. } => Default::default(),
            SynPatternData::Props { .. } => todo!(),
            SynPatternData::OneOf { ref options } => {
                debug_assert!(options.elements().len() > 1);
                let symbols = self.pattern_symbol_maps[options.elements()[0].syn_pattern()].clone();
                for option in &options.elements()[1..] {
                    let option_symbols = &self.pattern_symbol_maps[option.syn_pattern()];
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
        let modifier = symbol.pattern_symbol_modifier(&self.pattern_expr_arena);
        let idx = self.pattern_symbol_arena.alloc_one(symbol);
        self.pattern_symbol_modifiers.insert_next(idx, modifier);
        idx
    }

    pub fn pattern_exprs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (SynPatternIdx, &'a SynPatternData)> + 'a {
        self.pattern_expr_arena.indexed_iter()
    }

    pub fn pattern_expr_symbols(
        &self,
        syn_pattern_expr_idx: SynPatternIdx,
    ) -> &[(Ident, PatternVariableIdx)] {
        &self.pattern_symbol_maps[syn_pattern_expr_idx]
    }

    pub fn pattern_expr_arena(&self) -> &SynPatternArena {
        &self.pattern_expr_arena
    }

    pub fn pattern_symbol_arena(&self) -> &SynPatternSymbolArena {
        &self.pattern_symbol_arena
    }
}

impl std::ops::Index<SynPatternIdx> for SynPatternExprRegion {
    type Output = SynPatternData;

    fn index(&self, index: SynPatternIdx) -> &Self::Output {
        &self.pattern_expr_arena[index]
    }
}

impl std::ops::Index<PatternVariableIdx> for SynPatternExprRegion {
    type Output = PatternVariable;

    fn index(&self, index: PatternVariableIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl std::ops::Index<&PatternVariableIdx> for SynPatternExprRegion {
    type Output = PatternVariable;

    fn index(&self, index: &PatternVariableIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl SynExprRegionData {
    pub fn pattern_contract(&self, syn_pattern: SynPatternIdx) -> Contract {
        self.pattern_expr_region().pattern_contract(syn_pattern)
    }

    pub fn pattern_symbol_modifier(&self, pattern_symbol: PatternVariableIdx) -> VariableModifier {
        self.pattern_expr_region()
            .pattern_symbol_modifier(pattern_symbol)
    }
}

impl SynPatternExprRegion {
    fn pattern_contract(&self, pattern_expr_idx: SynPatternIdx) -> Contract {
        self.pattern_expr_contracts[pattern_expr_idx]
    }

    pub fn pattern_symbol_modifier(
        &self,
        pattern_variable_idx: PatternVariableIdx,
    ) -> VariableModifier {
        self.pattern_symbol_modifiers[pattern_variable_idx]
    }
}
