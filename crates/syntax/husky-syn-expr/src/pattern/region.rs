use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb, jar = SynExprJar)]
pub struct SynPatternExprRegion {
    pattern_expr_arena: SynPatternExprArena,
    /// the contract of pattern expressions are computed when they are created
    pattern_expr_contracts: SynPatternExprOrderedMap<Contract>,
    pattern_symbol_arena: SynPatternSymbolArena,
    pattern_symbol_maps: SynPatternExprOrderedMap<IdentPairMap<SynPatternSymbolIdx>>,
    pattern_symbol_modifiers: SynPatternSymbolOrderedMap<SymbolModifier>,
}

impl SynPatternExprRegion {
    pub fn alloc_one_pattern_expr(&mut self, expr: SynPatternExpr) -> SynPatternExprIdx {
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
        pattern_expr_idx: SynPatternExprIdx,
    ) -> IdentPairMap<SynPatternSymbolIdx> {
        let symbols: IdentPairMap<SynPatternSymbolIdx> = match self.pattern_expr_arena
            [pattern_expr_idx]
        {
            SynPatternExpr::Literal { .. } => Default::default(),
            SynPatternExpr::Ident { ident_token, .. } => IdentPairMap::new_one_element_map((
                ident_token.ident(),
                self.alloc_new_symbol(SynPatternSymbol::Atom(pattern_expr_idx)),
            )),
            SynPatternExpr::TypeVariantUnit { .. } => Default::default(),
            SynPatternExpr::Tuple { .. } => todo!(),
            SynPatternExpr::Props { .. } => todo!(),
            SynPatternExpr::OneOf { ref options } => {
                debug_assert!(options.elements().len() > 1);
                let symbols =
                    self.pattern_symbol_maps[options.elements()[0].syn_pattern_expr_idx()].clone();
                for option in &options.elements()[1..] {
                    let option_symbols = &self.pattern_symbol_maps[option.syn_pattern_expr_idx()];
                    if option_symbols != &symbols {
                        todo!()
                    }
                }
                symbols
            }
            SynPatternExpr::Binding { .. } => todo!(),
            SynPatternExpr::Range { .. } => todo!(),
        };
        symbols
    }

    fn alloc_new_symbol(&mut self, symbol: SynPatternSymbol) -> SynPatternSymbolIdx {
        let modifier = symbol.pattern_symbol_modifier(&self.pattern_expr_arena);
        let idx = self.pattern_symbol_arena.alloc_one(symbol);
        self.pattern_symbol_modifiers.insert_next(idx, modifier);
        idx
    }

    pub fn pattern_exprs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (SynPatternExprIdx, &'a SynPatternExpr)> + 'a {
        self.pattern_expr_arena.indexed_iter()
    }

    pub fn pattern_expr_symbols(
        &self,
        syn_pattern_expr_idx: SynPatternExprIdx,
    ) -> &[(Ident, SynPatternSymbolIdx)] {
        &self.pattern_symbol_maps[syn_pattern_expr_idx]
    }

    pub fn pattern_expr_arena(&self) -> &SynPatternExprArena {
        &self.pattern_expr_arena
    }

    pub fn pattern_symbol_arena(&self) -> &SynPatternSymbolArena {
        &self.pattern_symbol_arena
    }
}

impl std::ops::Index<SynPatternExprIdx> for SynPatternExprRegion {
    type Output = SynPatternExpr;

    fn index(&self, index: SynPatternExprIdx) -> &Self::Output {
        &self.pattern_expr_arena[index]
    }
}

impl std::ops::Index<SynPatternSymbolIdx> for SynPatternExprRegion {
    type Output = SynPatternSymbol;

    fn index(&self, index: SynPatternSymbolIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl std::ops::Index<&SynPatternSymbolIdx> for SynPatternExprRegion {
    type Output = SynPatternSymbol;

    fn index(&self, index: &SynPatternSymbolIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl SynExprRegionData {
    pub fn pattern_contract(&self, pattern_expr_idx: SynPatternExprIdx) -> Contract {
        self.pattern_expr_region()
            .pattern_contract(pattern_expr_idx)
    }

    pub fn pattern_symbol_modifier(
        &self,
        pattern_symbol_idx: SynPatternSymbolIdx,
    ) -> SymbolModifier {
        self.pattern_expr_region()
            .pattern_symbol_modifier(pattern_symbol_idx)
    }
}

impl SynPatternExprRegion {
    fn pattern_contract(&self, pattern_expr_idx: SynPatternExprIdx) -> Contract {
        self.pattern_expr_contracts[pattern_expr_idx]
    }

    pub fn pattern_symbol_modifier(
        &self,
        pattern_symbol_idx: SynPatternSymbolIdx,
    ) -> SymbolModifier {
        self.pattern_symbol_modifiers[pattern_symbol_idx]
    }
}
