use super::*;
use husky_hir_decl::decl::TypeAssocValHirDecl;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssocValHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssocValHirDecl,
    pub hir_expr_region: Option<HirExprRegion>,
}

impl From<TypeAssocValHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TypeAssocValHirDefn) -> Self {
        AssocItemHirDefn::TypeItem(hir_defn.into())
    }
}

impl From<TypeAssocValHirDefn> for HirDefn {
    fn from(hir_defn: TypeAssocValHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TypeAssocValHirDefn {
    pub(super) fn new(
        _db: &::salsa::Db,
        _path: TypeItemPath,
        _hir_decl: TypeAssocValHirDecl,
    ) -> Self {
        todo!()
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        ty_assoc_val_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_assoc_val_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_assoc_val_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TypeAssocValHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).impl_block(db));
    builder.add_hir_expr_region(hir_decl.hir_expr_region(db));
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_expr_region) = hir_defn.hir_expr_region(db) {
        builder.add_hir_expr_region(hir_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_assoc_val_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeAssocValHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
