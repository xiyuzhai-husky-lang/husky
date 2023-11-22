use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedValHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeAssociatedValHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitForTypeAssociatedValHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TraitForTypeItemPath,
        _hir_decl: TraitForTypeAssociatedValHirDecl,
    ) -> Self {
        todo!()
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        trai_for_ty_associated_val_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        trai_for_ty_associated_val_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_for_ty_associated_val_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TraitForTypeAssociatedValHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_expr_region) = hir_defn.hir_expr_region(db) {
        builder.add_hir_expr_region(hir_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_for_ty_associated_val_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TraitForTypeAssociatedValHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
