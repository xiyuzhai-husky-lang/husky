use super::*;
use husky_hir_decl::decl::TypeAssocTypeHirDecl;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssocTypeHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssocTypeHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
    pub hir_eager_expr_region: Option<HirEagerExprRegion>,
}

impl From<TypeAssocTypeHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TypeAssocTypeHirDefn) -> Self {
        AssocItemHirDefn::TypeItem(hir_defn.into())
    }
}

impl From<TypeAssocTypeHirDefn> for HirDefn {
    fn from(hir_defn: TypeAssocTypeHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TypeAssocTypeHirDefn {
    pub(super) fn new(
        _db: &::salsa::Db,
        _path: TypeItemPath,
        _hir_decl: TypeAssocTypeHirDecl,
    ) -> Self {
        todo!()
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        ty_assoc_ty_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_assoc_ty_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_assoc_ty_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TypeAssocTypeHirDefn,
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
fn ty_assoc_ty_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeAssocTypeHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
