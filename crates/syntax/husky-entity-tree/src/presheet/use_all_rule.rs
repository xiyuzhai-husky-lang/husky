use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct UseAllRule {
    parent: ModulePath,
    ast_idx: AstIdx,
    use_expr_idx: UseExprIdx,
    accessibility: Accessibility,
    // how many symbols have been checked
    progress: usize,
}

impl UseAllRule {
    pub fn new(
        parent: ModulePath,
        ast_idx: AstIdx,
        use_expr_idx: UseExprIdx,
        accessibility: Accessibility,
    ) -> Self {
        Self {
            parent,
            progress: 0,
            use_expr_idx,
            accessibility,
            ast_idx,
        }
    }

    pub fn parent(&self) -> ModulePath {
        self.parent
    }

    pub fn progress(&self) -> usize {
        self.progress
    }

    pub(crate) fn is_unresolved(&self, ctx: &EntityTreeSymbolContext) -> bool {
        self.progress < ctx.module_symbols(self.parent).len()
    }

    pub fn use_expr_idx(&self) -> ArenaIdx<UseExpr> {
        self.use_expr_idx
    }

    pub fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    pub fn ast_idx(&self) -> ArenaIdx<Ast> {
        self.ast_idx
    }

    pub(super) fn set_progress(&mut self, progress: usize) {
        self.progress = progress
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct UseAllRules(Vec<UseAllRule>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UseAllRuleIdx(usize);

impl UseAllRules {
    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseAllRuleIdx, &UseAllRule)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, rule)| (UseAllRuleIdx(i), rule))
    }

    pub(super) fn push(&mut self, new_rule: UseAllRule) {
        self.0.push(new_rule)
    }
}

impl std::ops::Index<UseAllRuleIdx> for UseAllRules {
    type Output = UseAllRule;

    fn index(&self, index: UseAllRuleIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<UseAllRuleIdx> for UseAllRules {
    fn index_mut(&mut self, index: UseAllRuleIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl<'a> std::ops::Index<UseAllRuleIdx> for EntityTreePresheetMut<'a> {
    type Output = UseAllRule;

    fn index(&self, index: UseAllRuleIdx) -> &Self::Output {
        &self.use_all_rules[index]
    }
}

impl<'a> std::ops::IndexMut<UseAllRuleIdx> for EntityTreePresheetMut<'a> {
    fn index_mut(&mut self, index: UseAllRuleIdx) -> &mut Self::Output {
        &mut self.use_all_rules[index]
    }
}
