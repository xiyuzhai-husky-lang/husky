use super::*;
use husky_hir_decl::decl::TraitForTypeAssocTypeHirDecl;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssocTypeHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssocTypeHirDecl,
}

impl From<TraitForTypeAssocTypeHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TraitForTypeAssocTypeHirDefn) -> Self {
        AssocItemHirDefn::TraitForTypeItem(hir_defn.into())
    }
}

impl From<TraitForTypeAssocTypeHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeAssocTypeHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TraitForTypeAssocTypeHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeAssocTypeHirDecl,
    ) -> Self {
        TraitForTypeAssocTypeHirDefn::new_inner(db, path, hir_decl)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_for_ty_associated_ty_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_for_ty_associated_ty_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_for_ty_associated_ty_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TraitForTypeAssocTypeHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).impl_block(db));
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.ty(db));
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_for_ty_associated_ty_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TraitForTypeAssocTypeHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
