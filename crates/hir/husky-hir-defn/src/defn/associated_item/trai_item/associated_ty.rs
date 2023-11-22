use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedTypeHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitAssociatedTypeHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitAssociatedTypeHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        trai_associated_ty_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        trai_associated_ty_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_ty_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedTypeHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_ty_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedTypeHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
