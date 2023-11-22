use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct FunctionFnHirDefn {
    pub path: FugitivePath,
    pub hir_decl: FunctionFnFugitiveHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl FunctionFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: FugitivePath,
        hir_decl: FunctionFnFugitiveHirDecl,
    ) -> Self {
        let Ok(FugitiveSynDefn::FunctionFn(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        FunctionFnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &dyn HirDefnDb) -> Option<HirEagerExprRegion> {
        Some(self.eager_body_with_hir_eager_expr_region(db)?.1)
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        function_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        function_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_fn_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: FunctionFnHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_fn_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: FunctionFnHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
