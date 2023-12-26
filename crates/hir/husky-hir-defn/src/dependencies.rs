use crate::*;
#[cfg(test)]
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_ty::{
    indirections::{HirIndirection, HirIndirections},
    instantiation::{HirInstantiation, HirTermSymbolResolution},
    ritchie::HirRitchieParameter,
    trai::HirTrait,
    HirTemplateArgument, HirType,
};
use vec_like::VecSet;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = new)]
pub(crate) struct HirDefnDependencies {
    #[return_ref]
    pub(crate) item_paths_in_current_crate: VecSet<ItemPath>,
    #[return_ref]
    pub(crate) item_paths_in_other_local_crates: VecSet<ItemPath>,
}

pub(crate) struct HirDefnDependenciesBuilder<'a> {
    item_path: ItemPath,
    crate_path: CratePath,
    item_paths_in_current_crate: VecSet<ItemPath>,
    item_paths_in_other_local_crates: VecSet<ItemPath>,
    db: &'a ::salsa::Db,
}

impl<'a> HirDefnDependenciesBuilder<'a> {
    pub(crate) fn new(item_path: impl Into<ItemPath>, db: &'a ::salsa::Db) -> Self {
        let item_path = item_path.into();
        Self {
            item_path,
            crate_path: item_path.crate_path(db),
            item_paths_in_current_crate: Default::default(),
            item_paths_in_other_local_crates: Default::default(),
            db,
        }
    }

    pub(crate) fn add_hir_expr_region(&mut self, expr_region: HirExprRegion) {
        match expr_region {
            HirExprRegion::Eager(hir_eager_expr_region) => {
                self.add_hir_eager_expr_region(hir_eager_expr_region)
            }
            HirExprRegion::Lazy(hir_lazy_expr_region) => {
                self.add_hir_lazy_expr_region(hir_lazy_expr_region)
            }
        }
    }

    pub(crate) fn add_hir_eager_expr_region(&mut self, hir_eager_expr_region: HirEagerExprRegion) {
        let db = self.db;
        for entry in hir_eager_expr_region.expr_arena(db) {
            match entry.data {
                HirEagerExprData::Literal(_) => (),
                HirEagerExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => self.add_item_path(path),
                    PrincipalEntityPath::TypeVariant(path) => {
                        self.add_item_path(path.parent_ty_path(db))
                    }
                },
                HirEagerExprData::ConstSymbol { .. } => (),
                HirEagerExprData::Variable(_) => (),
                HirEagerExprData::Binary { .. } => (),
                HirEagerExprData::Be { .. } => (),
                HirEagerExprData::Prefix { .. } => (),
                HirEagerExprData::Suffix { .. } => (),
                HirEagerExprData::Unveil {
                    unveil_associated_fn_path,
                    ..
                } => self.add_item_path(unveil_associated_fn_path),
                HirEagerExprData::Unwrap { .. } => (),
                HirEagerExprData::TypeConstructorFnCall { path, .. } => self.add_item_path(path),
                HirEagerExprData::TypeVariantConstructorCall { path, .. } => {
                    self.add_item_path(path.parent_ty_path(db))
                }
                HirEagerExprData::FunctionFnCall { path, .. } => self.add_item_path(path),
                HirEagerExprData::AssociatedFunctionFnCall { path, .. } => self.add_item_path(path),
                HirEagerExprData::PropsStructField { .. } => (),
                HirEagerExprData::MemoizedField { path, .. } => self.add_item_path(path),
                HirEagerExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    self.add_item_path(path);
                    self.add_instantiation(instantiation)
                }
                HirEagerExprData::NewTuple { .. } => (),
                HirEagerExprData::Index { .. } =>
                /* ad hoc */
                {
                    ()
                }
                // todo!(),
                HirEagerExprData::NewList { .. } => (),
                HirEagerExprData::Block { .. } => (),
                HirEagerExprData::EmptyHtmlTag { .. } => (),
                HirEagerExprData::Todo => (),
                HirEagerExprData::Unreachable => (),
                HirEagerExprData::AssociatedFn {
                    associated_item_path,
                } => self.add_item_path(associated_item_path),
                HirEagerExprData::As { opd, ty } => self.add_hir_ty(ty),
            }
        }
    }

    pub(crate) fn add_hir_lazy_expr_region(&mut self, hir_lazy_expr_region: HirLazyExprRegion) {
        let db = self.db;
        let hir_lazy_expr_arena = hir_lazy_expr_region.hir_lazy_expr_arena(db);
        for hir_lazy_expr_data in hir_lazy_expr_arena.iter() {
            match *hir_lazy_expr_data {
                HirLazyExprData::Literal(_) => (),
                HirLazyExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => self.add_item_path(path),
                    PrincipalEntityPath::TypeVariant(path) => {
                        self.add_item_path(path.parent_ty_path(db))
                    }
                },
                HirLazyExprData::ConstSymbol(_) => (),
                HirLazyExprData::Variable(_) => (),
                HirLazyExprData::Binary { .. } => (),
                HirLazyExprData::Be { .. } => (),
                HirLazyExprData::Prefix { .. } => (),
                HirLazyExprData::Suffix { .. } => (),
                HirLazyExprData::Unveil {
                    unveil_associated_fn_path,
                    ..
                } => self.add_item_path(unveil_associated_fn_path),
                HirLazyExprData::Unwrap { .. } => (),
                HirLazyExprData::TypeConstructorFnCall { path, .. } => self.add_item_path(path),
                HirLazyExprData::TypeVariantConstructorFnCall { path, .. } => {
                    self.add_item_path(path.parent_ty_path(db))
                }
                HirLazyExprData::FunctionFnItemCall { path, .. } => self.add_item_path(path),
                HirLazyExprData::FunctionGnItemCall { path, .. } => self.add_item_path(path),
                HirLazyExprData::AssociatedFunctionFnCall { path, .. } => self.add_item_path(path),
                HirLazyExprData::PropsStructField { .. } => (),
                HirLazyExprData::MemoizedField {
                    path,
                    ref indirections,
                    ref instantiation,
                    ..
                } => {
                    self.add_item_path(path);
                    self.add_indirections(indirections);
                    self.add_instantiation(instantiation)
                }
                HirLazyExprData::MethodFnCall { path, .. } => {
                    // todo!();
                    self.add_item_path(path)
                }
                HirLazyExprData::NewTuple { .. } => (),
                HirLazyExprData::Index { .. } =>
                /* ad hoc */
                {
                    ()
                }
                // todo!(),
                HirLazyExprData::ConstructList { .. } => (),
                HirLazyExprData::Block { .. } => (),
                HirLazyExprData::EmptyHtmlTag { .. } => (),
                HirLazyExprData::Todo => (),
                HirLazyExprData::Unreachable => (),
                HirLazyExprData::AssociatedFn { path } => self.add_item_path(path),
                HirLazyExprData::As { ty, .. } => self.add_hir_ty(ty),
            }
        }
    }

    pub(crate) fn add_hir_trai(&mut self, hir_trai: HirTrait) {
        let db = self.db;
        self.add_item_path(hir_trai.trai_path(db));
        self.add_hir_template_arguments(hir_trai.template_arguments(db))
    }

    pub(crate) fn add_hir_ty(&mut self, ty: HirType) {
        let db = self.db;
        match ty {
            HirType::PathLeading(hir_ty) => {
                self.add_item_path(hir_ty.ty_path(db));
                self.add_hir_template_arguments(hir_ty.template_arguments(db))
            }
            HirType::Symbol(_) => (),
            HirType::TypeAssociatedType(_) => (),
            HirType::TraitAssociatedType(_) => (),
            HirType::Ritchie(hir_ty) => {
                for param in hir_ty.parameters(db).iter() {
                    match param {
                        HirRitchieParameter::Regular(param) => self.add_hir_ty(param.ty()),
                        HirRitchieParameter::Variadic(_) => todo!(),
                        HirRitchieParameter::Keyed(_) => todo!(),
                    }
                }
                self.add_hir_ty(hir_ty.return_ty(db))
            }
        }
    }

    pub(crate) fn add_hir_template_arguments(&mut self, args: &[HirTemplateArgument]) {
        for arg in args {
            self.add_hir_template_argument(arg)
        }
    }

    pub(crate) fn add_hir_template_argument(&mut self, arg: &HirTemplateArgument) {
        match *arg {
            HirTemplateArgument::Vacant => (),
            HirTemplateArgument::Type(hir_ty) => self.add_hir_ty(hir_ty),
            HirTemplateArgument::Constant(_) => (),
            HirTemplateArgument::Lifetime(_) => (),
            HirTemplateArgument::Place(_) => (),
        }
    }

    // todo: consider instantiation also,
    // which might contain trait implementations
    pub(crate) fn add_item_path(&mut self, item_path: impl Into<ItemPath>) {
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

    pub(crate) fn finish(self) -> HirDefnDependencies {
        HirDefnDependencies::new(
            self.db,
            self.item_paths_in_current_crate,
            self.item_paths_in_other_local_crates,
        )
    }

    fn add_indirections(&mut self, indirections: &HirIndirections) {
        for indirection in indirections.iter() {
            match indirection {
                HirIndirection::Place(_) => (),
                HirIndirection::Leash => (),
            }
        }
    }

    fn add_instantiation(&mut self, instantiation: &HirInstantiation) {
        for (_, resolution) in instantiation.iter() {
            match resolution {
                HirTermSymbolResolution::Explicit(hir_template_argument) => {
                    self.add_hir_template_argument(hir_template_argument)
                }
                HirTermSymbolResolution::SelfLifetime => (),
                HirTermSymbolResolution::SelfPlace(_) => (),
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn module_hir_defn_dependencies(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> Vec<HirDefnDependencies> {
    module_item_paths(db, module_path)
        .iter()
        .filter_map(|path| path.hir_defn(db)?.dependencies(db))
        .collect()
}

#[test]
fn module_hir_defn_dependencies_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug(
        module_hir_defn_dependencies,
        &AstTestConfig::new("module_hir_defn_dependencies"),
    )
}
