use vec_like::VecMapGetEntry;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct UseAllRule {
    /// parent is of type `RelativeModulePath`
    ///
    /// because we would like to handle the two cases separately:
    /// - parent is inside the current crate
    /// - parent is outside the current crate
    parent: KinshipedModulePath,
    ast_idx: AstIdx,
    use_expr_idx: UseExprIdx,
    visibility: Visibility,
    // how many symbols have been checked
    progress: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct KinshipedModulePath {
    // precomputed and save here for efficiency
    kinship: ModulePathKinship,
    path: ModulePath,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModulePathKinship {
    /// module path inside the current crate
    Inside,
    /// module path outside the current crate
    Outside,
}

impl KinshipedModulePath {
    fn new(db: &dyn EntityTreeDb, crate_path: CratePath, path: ModulePath) -> Self {
        let kinship = match crate_path == path.crate_path(db) {
            true => ModulePathKinship::Inside,
            false => ModulePathKinship::Outside,
        };
        Self { kinship, path }
    }

    #[inline(always)]
    pub(crate) fn module_symbols<'a, 'b>(
        self,
        db: &'a dyn EntityTreeDb,
        presheets: &'b [EntityTreePresheetMut<'a>],
    ) -> EntityTreeResult<EntitySymbolTableRef<'b>>
    where
        'a: 'b,
    {
        Ok(match self.kinship {
            ModulePathKinship::Inside => presheets
                .get_entry(self.path)
                .expect("inside path should all be present in presheets")
                .module_specific_symbols(),
            ModulePathKinship::Outside => db.entity_tree_sheet(self.path)?.module_symbols(),
        })
    }
}

impl UseAllRule {
    pub(crate) fn new(
        db: &dyn EntityTreeDb,
        sheet: &EntityTreePresheetMut,
        parent: ModulePath,
        ast_idx: AstIdx,
        use_expr_idx: UseExprIdx,
        accessibility: Visibility,
    ) -> Self {
        Self {
            parent: KinshipedModulePath::new(db, sheet.module_path().crate_path(db), parent),
            progress: 0,
            use_expr_idx,
            visibility: accessibility,
            ast_idx,
        }
    }

    pub(crate) fn parent(&self) -> KinshipedModulePath {
        self.parent
    }

    pub fn progress(&self) -> usize {
        self.progress
    }

    pub(crate) fn is_unresolved(&self, ctx: &EntityTreeSymbolContext) -> bool {
        match self.parent.kinship {
            ModulePathKinship::Inside => {
                let Ok(module_symbols) = self.parent.module_symbols(ctx.db(), ctx.presheets()) else {
                    todo!()
                };
                self.progress < module_symbols.len()
            }
            ModulePathKinship::Outside => self.progress == 0,
        }
    }

    pub fn use_expr_idx(&self) -> UseExprIdx {
        self.use_expr_idx
    }

    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    pub fn ast_idx(&self) -> AstIdx {
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
