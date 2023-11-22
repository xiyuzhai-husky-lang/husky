use super::*;
use husky_hir_expr::helpers::hir_body_with_expr_region;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct ValHirDefn {
    pub path: FugitivePath,
    pub hir_decl: ValFugitiveHirDecl,
    pub body_with_hir_expr_region: Option<(HirExprIdx, HirExprRegion)>,
}

impl ValHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: FugitivePath,
        hir_decl: ValFugitiveHirDecl,
    ) -> Self {
        let Ok(FugitiveSynDefn::Val(syn_defn)) = path.syn_defn(db) else {
            unreachable!()
        };
        Self::new_inner(
            db,
            path,
            hir_decl,
            hir_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        self.body_with_hir_expr_region(db).map(|v| v.1)
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        val_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        val_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn val_hir_defn_dependencies(db: &dyn HirDefnDb, hir_defn: ValHirDefn) -> HirDefnDependencies {
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
fn val_hir_defn_version_stamp(db: &dyn HirDefnDb, hir_defn: ValHirDefn) -> HirDefnVersionStamp {
    todo!()
}
