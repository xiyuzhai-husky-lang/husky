use husky_hir_decl::parameter::parenate::eager::HirEagerParenateParameter;

use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FunctionFnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: FunctionFnFugitiveHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<FunctionFnHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: FunctionFnHirDefn) -> Self {
        MajorItemHirDefn::Fugitive(hir_defn.into())
    }
}

impl From<FunctionFnHirDefn> for HirDefn {
    fn from(hir_defn: FunctionFnHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl FunctionFnHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: FugitivePath,
        hir_decl: FunctionFnFugitiveHirDecl,
    ) -> Self {
        FunctionFnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(path.into(), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> Option<HirEagerExprRegion> {
        Some(self.eager_body_with_hir_eager_expr_region(db)?.1)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        function_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        function_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_fn_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: FunctionFnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for param in hir_decl.parenate_parameters(db).iter() {
        match *param {
            HirEagerParenateParameter::Ordinary { ty, .. } => builder.add_hir_ty(ty),
            HirEagerParenateParameter::Keyed => todo!(),
            HirEagerParenateParameter::Variadic => todo!(),
        }
    }
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_fn_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: FunctionFnHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
