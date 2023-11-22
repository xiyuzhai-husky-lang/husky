use super::*;

pub(super) struct ItemHirDefnDependenciesBuilder<'a> {
    item_path: ItemPath,
    crate_path: CratePath,
    item_paths_in_current_crate: VecSet<ItemPath>,
    item_paths_in_other_local_crates: VecSet<ItemPath>,
    db: &'a dyn HirDefnDb,
}

impl<'a> ItemHirDefnDependenciesBuilder<'a> {
    pub(super) fn new(
        item_path: ItemPath,
        hir_expr_region: HirExprRegion,
        db: &'a dyn HirDefnDb,
    ) -> Self {
        let mut slf = Self {
            item_path,
            crate_path: item_path.crate_path(db),
            item_paths_in_current_crate: Default::default(),
            item_paths_in_other_local_crates: Default::default(),
            db,
        };
        slf.add_hir_expr_region(hir_expr_region);
        slf
    }

    fn add_hir_expr_region(&mut self, hir_expr_region: HirExprRegion) {
        match hir_expr_region {
            HirExprRegion::Eager(hir_eager_expr_region) => {
                self.add_hir_eager_expr_region(hir_eager_expr_region)
            }
            HirExprRegion::Lazy(hir_lazy_expr_region) => todo!(),
        }
    }

    fn add_hir_eager_expr_region(&mut self, hir_eager_expr_region: HirEagerExprRegion) {
        let hir_eager_expr_arena = hir_eager_expr_region.hir_eager_expr_arena(self.db);
        for hir_eager_expr_data in hir_eager_expr_arena.iter() {
            match hir_eager_expr_data {
                HirEagerExprData::Literal(_) => todo!(),
                HirEagerExprData::PrincipalEntityPath(_) => todo!(),
                HirEagerExprData::ConstSymbol(_) => todo!(),
                HirEagerExprData::Variable(_) => todo!(),
                HirEagerExprData::Binary { lopd, opr, ropd } => todo!(),
                HirEagerExprData::Be { src, target } => todo!(),
                HirEagerExprData::Prefix {
                    opr,
                    opd_hir_expr_idx,
                } => todo!(),
                HirEagerExprData::Suffix {
                    opd_hir_expr_idx,
                    opr,
                } => todo!(),
                HirEagerExprData::TypeConstructorFnCall {
                    path,
                    function_hir_eager_expr_idx,
                    template_arguments,
                    item_groups,
                } => todo!(),
                HirEagerExprData::TypeVariantConstructorCall {
                    path,
                    function_hir_eager_expr_idx,
                    template_arguments,
                    item_groups,
                } => todo!(),
                HirEagerExprData::FunctionFnCall {
                    path,
                    function_hir_eager_expr_idx,
                    template_arguments,
                    item_groups,
                } => todo!(),
                HirEagerExprData::AssociatedFunctionFnCall {
                    path,
                    function_hir_eager_expr_idx,
                    parent_template_arguments,
                    template_arguments,
                    item_groups,
                } => todo!(),
                HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx,
                    ident,
                } => todo!(),
                HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx,
                    ident,
                } => todo!(),
                HirEagerExprData::MethodFnCall {
                    self_argument,
                    ident,
                    path,
                    template_arguments,
                    item_groups,
                } => todo!(),
                HirEagerExprData::NewTuple { items } => todo!(),
                HirEagerExprData::Index {
                    owner_hir_expr_idx,
                    items,
                } => todo!(),
                HirEagerExprData::NewList { items } => todo!(),
                HirEagerExprData::Block { stmts } => todo!(),
                HirEagerExprData::EmptyHtmlTag {
                    function_ident,
                    arguments,
                } => todo!(),
                HirEagerExprData::Todo => todo!(),
                HirEagerExprData::Unreachable => todo!(),
                HirEagerExprData::AssociatedFn {
                    associated_item_path,
                } => todo!(),
            }
        }
    }

    fn add_item_path(&mut self, item_path: impl Into<ItemPath>) {
        let item_path: ItemPath = item_path.into();
        if item_path == self.item_path {
            // no need to add self
            return;
        }
        let db = self.db;
        let crate_path = item_path.crate_path(db);
        if crate_path == self.crate_path {
            self.item_paths_in_current_crate.insert(item_path)
        } else {
            match crate_path.package_path(db).data(db) {
                PackagePathSource::Library => (),
                PackagePathSource::Registry { .. } => (),
                PackagePathSource::Git { .. } => (),
                PackagePathSource::Local { .. } => {
                    self.item_paths_in_other_local_crates.insert(item_path)
                }
            }
        }
    }

    fn finish(self) -> ItemHirDefnDependencies {
        ItemHirDefnDependencies {
            item_paths_in_current_crate: self.item_paths_in_current_crate,
            item_paths_in_other_local_crates: self.item_paths_in_other_local_crates,
        }
    }
}
