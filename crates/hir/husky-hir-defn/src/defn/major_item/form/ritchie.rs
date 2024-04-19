use super::*;
use husky_hir_decl::{
    decl::MajorRitchieHirDecl,
    parameter::parenate::{
        eager::HirEagerParenateParameter, lazy::HirLazyParenateParameter, HirParenateParameters,
    },
};
use husky_hir_expr::helpers::hir_body_with_expr_region;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct MajorRitchieHirDefn {
    pub path: MajorFormPath,
    pub hir_decl: MajorRitchieHirDecl,
    pub body_with_hir_expr_region: Option<(HirExprIdx, HirExprRegion)>,
}

impl From<MajorRitchieHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: MajorRitchieHirDefn) -> Self {
        MajorItemHirDefn::Form(hir_defn.into())
    }
}

impl From<MajorRitchieHirDefn> for HirDefn {
    fn from(hir_defn: MajorRitchieHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl MajorRitchieHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: MajorFormPath,
        hir_decl: MajorRitchieHirDecl,
    ) -> Self {
        MajorRitchieHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_body_with_expr_region(path.into(), db),
        )
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        Some(self.body_with_hir_expr_region(db)?.1)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        major_ritchie_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        function_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn major_ritchie_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: MajorRitchieHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_expr_region(hir_decl.hir_expr_region(db));
    match hir_decl.parenate_parameters(db) {
        HirParenateParameters::Eager(parenate_parameters) => {
            for param in parenate_parameters.iter() {
                match *param {
                    HirEagerParenateParameter::Simple { ty, .. } => builder.add_hir_ty(ty),
                    HirEagerParenateParameter::Keyed => todo!(),
                    HirEagerParenateParameter::Variadic => todo!(),
                }
            }
        }
        HirParenateParameters::Lazy(parenate_parameters) => {
            for param in parenate_parameters.iter() {
                match *param {
                    HirLazyParenateParameter::Simple { ty, .. } => builder.add_hir_ty(ty),
                    HirLazyParenateParameter::SelfValue => todo!(),
                    HirLazyParenateParameter::Keyed { ident, ty } => builder.add_hir_ty(ty),
                    HirLazyParenateParameter::Variadic { variant, ty } => builder.add_hir_ty(ty),
                }
            }
        }
    }
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_expr_region) = hir_defn.hir_expr_region(db) {
        builder.add_hir_expr_region(hir_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn function_fn_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: MajorRitchieHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
