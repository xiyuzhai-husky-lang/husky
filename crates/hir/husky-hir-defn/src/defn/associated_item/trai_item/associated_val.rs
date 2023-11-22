use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValHirDefn {
    pub syn_node_path: TraitItemPath,
    pub hir_decl: TraitAssociatedValHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TraitAssociatedValHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        trai_associated_val_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        trai_associated_val_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_val_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedValHirDefn,
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
fn trai_associated_val_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedValHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
