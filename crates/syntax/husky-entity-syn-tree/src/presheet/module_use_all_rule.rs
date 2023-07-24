use super::*;
use vec_like::VecMapGetEntry;

/// it's separated because it has to be updated indefinitely
#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct UseAllModuleSymbolsRule {
    /// parent is of type `RelativeModulePath`
    ///
    /// because we would like to handle the two cases separately:
    /// - parent is inside the current crate
    /// - parent is outside the current crate
    parent_module_path: ModulePath,
    is_same_crate: bool,
    ast_idx: AstIdx,
    use_expr_idx: UseExprIdx,
    visibility: Scope,
    // how many symbols have been checked
    progress: Result<usize, ()>,
}

impl UseAllModuleSymbolsRule {
    pub(crate) fn new(
        db: &dyn EntitySynTreeDb,
        sheet: &EntityTreePresheetMut,
        parent_module_path: ModulePath,
        ast_idx: AstIdx,
        use_expr_idx: UseExprIdx,
        visibility: Scope,
    ) -> Self {
        Self {
            parent_module_path,
            is_same_crate: parent_module_path.crate_path(db) == sheet.module_path.crate_path(db),
            progress: Ok(0),
            use_expr_idx,
            visibility,
            ast_idx,
        }
    }

    pub(crate) fn parent_path(&self) -> ModulePath {
        self.parent_module_path
    }

    pub fn progress(&self) -> Result<usize, ()> {
        self.progress
    }

    pub(crate) fn is_unresolved(&self, ctx: &EntityTreeSymbolContext) -> bool {
        let Ok(progress) = self.progress else {
            return false;
        };
        if self.is_same_crate {
            progress
                < ctx.presheets()[self.parent_module_path]
                    .module_specific_symbols()
                    .len()
        } else {
            progress == 0
        }
    }

    pub(crate) fn parent_module_specific_symbols<'a>(
        &self,
        db: &'a dyn EntitySynTreeDb,
        presheets: &'a [EntityTreePresheetMut],
    ) -> ItemSynTreeResult<EntitySymbolTableRef<'a>> {
        if self.is_same_crate {
            // avoids cyclic dependency
            Ok(presheets
                .get_entry(self.parent_module_path)
                .unwrap()
                .module_specific_symbols())
        } else {
            Ok(db
                .item_syn_tree_sheet(self.parent_module_path)?
                .module_symbols())
        }
    }

    pub fn use_expr_idx(&self) -> UseExprIdx {
        self.use_expr_idx
    }

    pub fn visibility(&self) -> Scope {
        self.visibility
    }

    pub fn ast_idx(&self) -> AstIdx {
        self.ast_idx
    }

    pub(super) fn set_progress(&mut self, progress: usize) {
        self.progress = Ok(progress)
    }

    pub(super) fn mark_as_erroneous(&mut self) {
        self.progress = Err(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub(crate) struct UseAllModuleSymbolsRules(Vec<UseAllModuleSymbolsRule>);

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UseAllModuleSymbolsRuleIdx(usize);

impl UseAllModuleSymbolsRules {
    pub(crate) fn indexed_iter(
        &self,
    ) -> impl Iterator<Item = (UseAllModuleSymbolsRuleIdx, &UseAllModuleSymbolsRule)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, rule)| (UseAllModuleSymbolsRuleIdx(i), rule))
    }

    pub(super) fn push(&mut self, new_rule: UseAllModuleSymbolsRule) {
        self.0.push(new_rule)
    }
}

impl std::ops::Index<UseAllModuleSymbolsRuleIdx> for UseAllModuleSymbolsRules {
    type Output = UseAllModuleSymbolsRule;

    fn index(&self, index: UseAllModuleSymbolsRuleIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl std::ops::IndexMut<UseAllModuleSymbolsRuleIdx> for UseAllModuleSymbolsRules {
    fn index_mut(&mut self, index: UseAllModuleSymbolsRuleIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}

impl<'a> std::ops::Index<UseAllModuleSymbolsRuleIdx> for EntityTreePresheetMut<'a> {
    type Output = UseAllModuleSymbolsRule;

    fn index(&self, index: UseAllModuleSymbolsRuleIdx) -> &Self::Output {
        &self.all_module_items_use_rules[index]
    }
}

impl<'a> std::ops::IndexMut<UseAllModuleSymbolsRuleIdx> for EntityTreePresheetMut<'a> {
    fn index_mut(&mut self, index: UseAllModuleSymbolsRuleIdx) -> &mut Self::Output {
        &mut self.all_module_items_use_rules[index]
    }
}
