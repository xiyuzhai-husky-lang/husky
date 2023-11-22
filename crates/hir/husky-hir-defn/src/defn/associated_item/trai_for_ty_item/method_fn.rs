use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitForTypeMethodFnHirDefn {
    pub path: TraitForTypeItemPath,
    pub hir_decl: TraitForTypeMethodFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TraitForTypeMethodFnHirDefn> for AssociatedItemHirDefn {
    fn from(hir_defn: TraitForTypeMethodFnHirDefn) -> Self {
        AssociatedItemHirDefn::TraitForTypeItem(hir_defn.into())
    }
}

impl From<TraitForTypeMethodFnHirDefn> for HirDefn {
    fn from(hir_defn: TraitForTypeMethodFnHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TraitForTypeMethodFnHirDefn {
    pub(super) fn new(
        db: &dyn HirDefnDb,
        path: TraitForTypeItemPath,
        hir_decl: TraitForTypeMethodFnHirDecl,
    ) -> TraitForTypeMethodFnHirDefn {
        let TraitForTypeItemSynDefn::MethodFn(syn_defn) =
            path.syn_defn(db).expect("hir stage no error")
        else {
            unreachable!()
        };
        TraitForTypeMethodFnHirDefn::new_inner(
            db,
            path,
            hir_decl,
            hir_eager_body_with_expr_region(syn_defn.body_with_syn_expr_region(db), db),
        )
    }

    pub fn hir_eager_expr_region(self, db: &dyn HirDefnDb) -> Option<HirEagerExprRegion> {
        self.eager_body_with_hir_eager_expr_region(db)
            .map(|(_, region)| region)
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        trai_for_ty_method_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        trai_for_ty_method_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_for_ty_method_fn_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TraitForTypeMethodFnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_item_path(hir_decl.path(db).impl_block(db));
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
fn trai_for_ty_method_fn_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TraitForTypeMethodFnHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
