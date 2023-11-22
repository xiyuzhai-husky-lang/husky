use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedTypeHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
    pub hir_eager_expr_region: Option<HirEagerExprRegion>,
}

impl From<TypeAssociatedTypeHirDefn> for AssociatedItemHirDefn {
    fn from(hir_defn: TypeAssociatedTypeHirDefn) -> Self {
        AssociatedItemHirDefn::TypeItem(hir_defn.into())
    }
}

impl From<TypeAssociatedTypeHirDefn> for HirDefn {
    fn from(hir_defn: TypeAssociatedTypeHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TypeAssociatedTypeHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TypeItemPath,
        _hir_decl: TypeAssociatedTypeHirDecl,
    ) -> Self {
        todo!()
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        ty_associated_ty_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        ty_associated_ty_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_associated_ty_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TypeAssociatedTypeHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).impl_block(db));
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_associated_ty_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TypeAssociatedTypeHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
