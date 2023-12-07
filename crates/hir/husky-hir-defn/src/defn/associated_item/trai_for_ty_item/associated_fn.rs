use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedFnHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TraitForTypeAssociatedFnHirDefn> for AssociatedItemHirDefn {
    fn from(hir_defn: TraitForTypeAssociatedFnHirDefn) -> Self {
        AssociatedItemHirDefn::TraitForTypeItem(hir_defn.into())
    }
}

impl From<TraitForTypeAssociatedFnHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeAssociatedFnHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TraitForTypeAssociatedFnHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeAssociatedFnHirDecl,
    ) -> Self {
        Self::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(path.into(), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> Option<HirEagerExprRegion> {
        self.eager_body_with_hir_eager_expr_region(db)
            .map(|(_, region)| region)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_for_ty_associated_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_for_ty_associated_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_for_ty_associated_fn_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TraitForTypeAssociatedFnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).impl_block(db));
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
fn trai_for_ty_associated_fn_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TraitForTypeAssociatedFnHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
