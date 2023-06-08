use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct PatternExprRegion {
    pattern_expr_arena: PatternExprArena,
    pattern_expr_contracts: PatternExprOrderedMap<Contract>,
    pattern_infos: Vec<PatternExprInfo>,
    pattern_symbol_arena: PatternSymbolArena,
    pattern_symbol_maps: PatternExprOrderedMap<IdentPairMap<PatternSymbolIdx>>,
    pattern_symbol_modifiers: PatternSymbolOrderedMap<SymbolModifier>,
}

impl PatternExprRegion {
    pub fn alloc_one_pattern_expr(
        &mut self,
        expr: PatternExpr,
        env: PatternExprInfo,
    ) -> PatternExprIdx {
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
        pattern_expr_idx: PatternExprIdx,
    ) -> IdentPairMap<PatternSymbolIdx> {
        let symbols: IdentPairMap<PatternSymbolIdx> =
            match self.pattern_expr_arena[pattern_expr_idx] {
                PatternExpr::Literal(_) => Default::default(),
                PatternExpr::Ident {
                    ident_token,
                    modifier_keyword_group: contract,
                } => IdentPairMap::new_one_element_map((
                    ident_token.ident(),
                    self.alloc_new_symbol(PatternSymbol::Atom(pattern_expr_idx)),
                )),
                PatternExpr::Entity(_) => todo!(),
                PatternExpr::Tuple { name, fields } => todo!(),
                PatternExpr::Struct { name, fields } => todo!(),
                PatternExpr::OneOf { options } => todo!(),
                PatternExpr::Binding {
                    ident_token,
                    asperand_token,
                    src,
                } => todo!(),
                PatternExpr::Range {
                    start,
                    dot_dot_token,
                    end,
                } => todo!(),
            };
        symbols
    }

    fn alloc_new_symbol(&mut self, symbol: PatternSymbol) -> PatternSymbolIdx {
        let modifier = symbol.modifier(&self.pattern_expr_arena);
        let idx = self.pattern_symbol_arena.alloc_one(symbol);
        self.pattern_symbol_modifiers.insert_next(idx, modifier);
        idx
    }

    pub fn pattern_exprs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (PatternExprIdx, &'a PatternExpr)> + 'a {
        self.pattern_expr_arena.indexed_iter()
    }

    pub fn pattern_expr_symbols(
        &self,
        pattern_expr_idx: PatternExprIdx,
    ) -> &[(Ident, PatternSymbolIdx)] {
        &self.pattern_symbol_maps[pattern_expr_idx]
    }

    pub fn pattern_info(&self, pattern_expr_idx: PatternExprIdx) -> PatternExprInfo {
        self.pattern_infos[pattern_expr_idx.raw()]
    }

    pub fn pattern_expr_arena(&self) -> &PatternExprArena {
        &self.pattern_expr_arena
    }

    pub fn pattern_symbol_arena(&self) -> &PatternSymbolArena {
        &self.pattern_symbol_arena
    }
}

impl std::ops::Index<PatternExprIdx> for PatternExprRegion {
    type Output = PatternExpr;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.pattern_expr_arena[index]
    }
}

impl std::ops::Index<PatternSymbolIdx> for PatternExprRegion {
    type Output = PatternSymbol;

    fn index(&self, index: PatternSymbolIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl std::ops::Index<&PatternSymbolIdx> for PatternExprRegion {
    type Output = PatternSymbol;

    fn index(&self, index: &PatternSymbolIdx) -> &Self::Output {
        &self.pattern_symbol_arena[index]
    }
}

impl ExprRegionData {
    pub fn pattern_contract(&self, pattern_expr_idx: PatternExprIdx) -> Contract {
        self.pattern_expr_region()
            .pattern_contract(pattern_expr_idx)
    }

    pub fn pattern_symbol_modifier(&self, pattern_symbol_idx: PatternSymbolIdx) -> SymbolModifier {
        self.pattern_expr_region()
            .pattern_symbol_modifier(pattern_symbol_idx)
    }
}

impl PatternExprRegion {
    fn pattern_contract(&self, pattern_expr_idx: PatternExprIdx) -> Contract {
        self.pattern_expr_contracts[pattern_expr_idx]
    }

    pub fn pattern_symbol_modifier(&self, pattern_symbol_idx: PatternSymbolIdx) -> SymbolModifier {
        self.pattern_symbol_modifiers[pattern_symbol_idx]
    }
}
