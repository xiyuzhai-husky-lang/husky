use super::*;
use husky_hir_decl::decl::TypeMethodRitchieHirDecl;

#[salsa::interned(constructor = new_inner)]
pub struct TypeMethodRitchieHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeMethodRitchieHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TypeMethodRitchieHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TypeMethodRitchieHirDefn) -> Self {
        AssocItemHirDefn::TypeItem(hir_defn.into())
    }
}

impl From<TypeMethodRitchieHirDefn> for HirDefn {
    fn from(hir_defn: TypeMethodRitchieHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TypeMethodRitchieHirDefn {
    pub(super) fn new(
        db: &::salsa::Db,
        path: TypeItemPath,
        hir_decl: TypeMethodRitchieHirDecl,
    ) -> Self {
        TypeMethodRitchieHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(path.into(), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> Option<HirEagerExprRegion> {
        Some(self.eager_body_with_hir_eager_expr_region(db)?.1)
    }

    pub(super) fn deps(self, db: &::salsa::Db) -> HirDefnDeps {
        ty_method_ritchie_hir_defn_deps(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_method_ritchie_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn ty_method_ritchie_hir_defn_deps(
    db: &::salsa::Db,
    hir_defn: TypeMethodRitchieHirDefn,
) -> HirDefnDeps {
    let mut builder = HirDefnDepsBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).impl_block(db));
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for param in hir_decl.parenate_parameters(db).iter() {
        match *param {
            HirEagerParenateParameter::Simple { ty, .. } => builder.add_hir_ty(ty),
            HirEagerParenateParameter::Keyed => todo!(),
            HirEagerParenateParameter::Variadic => todo!(),
        }
    }
    builder.add_hir_ty(hir_decl.return_ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked]
fn ty_method_ritchie_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeMethodRitchieHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
