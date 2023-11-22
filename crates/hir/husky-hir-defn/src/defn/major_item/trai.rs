use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TraitHirDefn {
    pub path: TraitPath,
    pub hir_decl: TraitHirDecl,
}

impl HasHirDefn for TraitPath {
    type HirDefn = TraitHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        trai_hir_defn(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
pub(crate) fn trai_hir_defn(db: &dyn HirDefnDb, path: TraitPath) -> Option<TraitHirDefn> {
    let hir_decl = path.hir_decl(db)?;
    Some(TraitHirDefn::new(db, path, hir_decl))
}

impl TraitHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        trai_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        trai_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_hir_defn_dependencies(db: &dyn HirDefnDb, hir_defn: TraitHirDefn) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_hir_defn_version_stamp(db: &dyn HirDefnDb, hir_defn: TraitHirDefn) -> HirDefnVersionStamp {
    todo!()
}
