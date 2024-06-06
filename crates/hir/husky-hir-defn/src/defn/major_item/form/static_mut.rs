use super::*;
use husky_hir_decl::decl::major_item::form::static_mut::MajorStaticMutHirDecl;
use husky_hir_expr::helpers::hir_body_with_expr_region;

#[salsa::interned(constructor = new_inner)]
pub struct MajorStaticMutHirDefn {
    pub path: MajorFormPath,
    pub hir_decl: MajorStaticMutHirDecl,
    pub hir_expr_body_and_region: Option<(HirExprIdx, HirExprRegion)>,
}

impl From<MajorStaticMutHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: MajorStaticMutHirDefn) -> Self {
        MajorItemHirDefn::Form(hir_defn.into())
    }
}

impl From<MajorStaticMutHirDefn> for HirDefn {
    fn from(hir_defn: MajorStaticMutHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl MajorStaticMutHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: MajorFormPath,
        hir_decl: MajorStaticMutHirDecl,
    ) -> Self {
        Self::new_inner(
            db,
            path,
            hir_decl,
            hir_body_with_expr_region(path.into(), db),
        )
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        self.hir_expr_body_and_region(db).map(|v| v.1)
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        major_static_mut_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        major_static_mut_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn major_static_mut_hir_defn_deps(
    db: &::salsa::Db,
    hir_defn: MajorStaticMutHirDefn,
) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_expr_region) = hir_defn.hir_expr_region(db) {
        builder.add_hir_expr_region(hir_expr_region);
    }
    builder.finish()
}

#[salsa::tracked]
fn major_static_mut_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: MajorStaticMutHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
