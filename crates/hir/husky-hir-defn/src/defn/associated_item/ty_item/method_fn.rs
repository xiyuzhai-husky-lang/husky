use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeMethodFnHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeMethodFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl TypeMethodFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TypeItemPath,
        hir_decl: TypeMethodFnHirDecl,
    ) -> Self {
        let Ok(TypeItemSynDefn::MethodFn(syn_defn)) = path.syn_defn(db) else {
            unreachable!("hir stage no error")
        };
        TypeMethodFnHirDefn::new_inner(
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
        ty_method_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        ty_method_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_method_fn_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TypeMethodFnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for param in hir_decl.parenate_parameters(db).iter() {
        match *param {
            HirEagerParenateParameter::Ordinary { ty, .. } => builder.add_hir_ty(ty),
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

#[salsa::tracked(jar = HirDefnJar)]
fn ty_method_fn_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TypeMethodFnHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
