use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub struct PatternSynExprRegion {
    pattern_expr_arena: PatternSynExprArena,
    pattern_expr_contracts: PatternSynExprOrderedMap<Contract>,
    pattern_infos: Vec<PatternSynExprInfo>,
    pattern_symbol_arena: PatternSymbolArena,
    pattern_symbol_maps: PatternSynExprOrderedMap<IdentPairMap<PatternSymbolIdx>>,
    pattern_symbol_modifiers: PatternSymbolOrderedMap<SymbolModifier>,
}

impl PatternSynExprRegion {
    pub fn alloc_one_pattern_expr(
        &mut self,
        expr: PatternSynExpr,
        env: PatternSynExprInfo,
    ) -> PatternSynExprIdx {
        // order matters
        let contract = expr.contract();
        let idx = self.pattern_expr_arena.alloc_one(expr);
        assert_eq!(idx.raw(), self.pattern_infos.len());
        self.pattern_infos.push(env);
        let symbols = self.collect_symbols(idx);
        assert_eq!(idx.raw(), self.pattern_symbol_maps.len());
        self.pattern_symbol_maps.insert_next(idx, symbols);
        self.pattern_expr_contracts.insert_next(idx, contract);
        idx
    }

    // expr must be allocated already
    fn collect_symbols(
        &mut self,
        pattern_expr_idx: PatternSynExprIdx,
    ) -> IdentPairMap<PatternSymbolIdx> {
        let symbols: IdentPairMap<PatternSymbolIdx> =
            match self.pattern_expr_arena[pattern_expr_idx] {
                PatternSynExpr::Literal(_) => Default::default(),
                PatternSynExpr::Ident {
                    ident_token,
                    symbol_modifier_keyword_group: contract,
                } => IdentPairMap::new_one_element_map((
                    ident_token.ident(),
                    self.alloc_new_symbol(PatternSynSymbol::Atom(pattern_expr_idx)),
                )),
                PatternSynExpr::Entity(_) => todo!(),
                PatternSynExpr::Tuple { name, fields } => todo!(),
                PatternSynExpr::Struct { name, fields } => todo!(),
                PatternSynExpr::OneOf { options } => todo!(),
                PatternSynExpr::Binding {
                    ident_token,
                    asperand_token,
                    src,
                } => todo!(),
                PatternSynExpr::Range {
                    start,
                    dot_dot_token,
                    end,
                } => todo!(),
            };
        symbols
    }

    fn alloc_new_symbol(&mut self, symbol: PatternSynSymbol) -> PatternSymbolIdx {
        let modifier = symbol.pattern_symbol_modifier(&self.pattern_expr_arena);
        let idx = self.pattern_symbol_arena.alloc_one(symbol);
        self.pattern_symbol_modifiers.insert_next(idx, modifier);
        idx
    }

    pub fn pattern_exprs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (PatternSynExprIdx, &'a PatternSynExpr)> + 'a {
        self.pattern_expr_arena.indexed_iter()
    }

    pub fn pattern_expr_symbols(
        &self,
        pattern_expr_idx: PatternSynExprIdx,
    ) -> &[(Ident, PatternSymbolIdx)] {
        &self.pattern_symbol_maps[pattern_expr_idx]
    }

    pub fn pattern_info(&self, pattern_expr_idx: PatternSynExprIdx) -> PatternSynExprInfo {
        self.pattern_infos[pattern_expr_idx.raw()]
    }

    pub fn pattern_expr_arena(&self) -> &PatternSynExprArena {
        &self.pattern_expr_arena
    }

    pub fn pattern_symbol_arena(&self) -> &PatternSymbolArena {
        &self.pattern_symbol_arena
    }
}

impl std::ops::Index<PatternSynExprIdx> for PatternSynExprRegion {
    type Output = PatternSynExpr;

    fn index(&self, index: PatternSynExprIdx) -> &Self::Output {
        &self.pattern_expr_arena[index]
    }
}

impl std::ops::Index<PatternSymbolIdx> for PatternSynExprRegion {
    type Output = PatternSynSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl std::ops::Index<&PatternSymbolIdx> for PatternSynExprRegion {
    type Output = PatternSynSymbol;

    fn index(&self, index: &PatternSymbolIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl SynExprRegionData {
    pub fn pattern_contract(&self, pattern_expr_idx: PatternSynExprIdx) -> Contract {
        self.pattern_expr_region()
            .pattern_contract(pattern_expr_idx)
    }

    pub fn pattern_symbol_modifier(&self, pattern_symbol_idx: PatternSymbolIdx) -> SymbolModifier {
        self.pattern_expr_region()
            .pattern_symbol_modifier(pattern_symbol_idx)
    }
}

impl PatternSynExprRegion {
    fn pattern_contract(&self, pattern_expr_idx: PatternSynExprIdx) -> Contract {
        self.pattern_expr_contracts[pattern_expr_idx]
    }

    pub fn pattern_symbol_modifier(&self, pattern_symbol_idx: PatternSymbolIdx) -> SymbolModifier {
        self.pattern_symbol_modifiers[pattern_symbol_idx]
    }
}
