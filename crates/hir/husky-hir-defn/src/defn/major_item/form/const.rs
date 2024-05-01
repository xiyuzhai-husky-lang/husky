use super::*;
use husky_hir_decl::decl::major_item::form::r#const::MajorConstHirDecl;
use husky_hir_expr::helpers::hir_body_with_expr_region;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct MajorConstHirDefn {
    pub path: MajorFormPath,
    pub hir_decl: MajorConstHirDecl,
    pub hir_expr_body_and_region: Option<(HirExprIdx, HirExprRegion)>,
}

impl From<MajorConstHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: MajorConstHirDefn) -> Self {
        MajorItemHirDefn::Form(hir_defn.into())
    }
}

impl From<MajorConstHirDefn> for HirDefn {
    fn from(hir_defn: MajorConstHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl MajorConstHirDefn {
    pub(super) fn new(db: &::salsa::Db, path: MajorFormPath, hir_decl: MajorConstHirDecl) -> Self {
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

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        const_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        const_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn const_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: MajorConstHirDefn,
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
fn const_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: MajorConstHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
