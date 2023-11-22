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
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_val_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedValHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
