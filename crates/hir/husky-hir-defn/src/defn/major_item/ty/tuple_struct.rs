use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TupleStructHirDefn {
    pub path: TypePath,
    pub hir_decl: TupleStructTypeHirDecl,
}

impl TupleStructHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        tuple_struct_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        tuple_struct_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn tuple_struct_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TupleStructHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn tuple_struct_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TupleStructHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
