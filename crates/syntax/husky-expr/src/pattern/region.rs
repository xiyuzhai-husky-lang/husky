use super::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprDb)]
pub struct PatternExprRegion {
    pattern_expr_arena: PatternExprArena,
    pattern_infos: Vec<PatternExprInfo>,
    pattern_symbol_maps: Vec<IdentPairMap<PatternSymbolIdx>>,
    pattern_symbol_arena: PatternSymbolArena,
}

impl PatternExprRegion {
    pub fn alloc_one_pattern_expr(
        &mut self,
        expr: PatternExpr,
        env: PatternExprInfo,
    ) -> PatternExprIdx {
        let expr_idx = self.pattern_expr_arena.alloc_one(expr);
        assert_eq!(expr_idx.raw(), self.pattern_infos.len());
        self.pattern_infos.push(env);
        let expr = &self.pattern_expr_arena[expr_idx];
        assert_eq!(expr_idx.raw(), self.pattern_symbol_maps.len());
        self.pattern_symbol_maps.push(collect_symbols(
            expr_idx,
            expr,
            &mut self.pattern_symbol_arena,
        ));
        expr_idx
    }

    pub fn pattern_exprs<'a>(
        &'a self,
    ) -> impl Iterator<Item = (PatternExprIdx, &'a PatternExpr)> + 'a {
        self.pattern_expr_arena.indexed_iter()
    }

    pub fn pattern_symbol_map(
        &self,
        pattern_expr_idx: ArenaIdx<PatternExpr>,
    ) -> &IdentPairMap<PatternSymbolIdx> {
        &self.pattern_symbol_maps[pattern_expr_idx.raw()]
    }

    pub fn pattern_info(&self, pattern_expr_idx: ArenaIdx<PatternExpr>) -> PatternExprInfo {
        self.pattern_infos[pattern_expr_idx.raw()]
    }

    pub fn pattern_expr_arena(&self) -> &PatternExprArena {
        &self.pattern_expr_arena
    }

    pub fn pattern_symbol_arena(&self) -> &PatternSymbolArena {
        &self.pattern_symbol_arena
    }
}

fn collect_symbols(
    pattern_expr_idx: PatternExprIdx,
    pattern_expr: &PatternExpr,
    pattern_symbol_arena: &mut PatternSymbolArena,
) -> IdentPairMap<PatternSymbolIdx> {
    match pattern_expr {
        PatternExpr::Literal(_) => Default::default(),
        PatternExpr::Identifier {
            ident_token,
            liason,
        } => [(
            ident_token.ident(),
            pattern_symbol_arena.alloc_one(PatternSymbol::Atom(pattern_expr_idx)),
        )]
        .into(),
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
