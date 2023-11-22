use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssociatedFnHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitAssociatedFnHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TraitAssociatedFnHirDefn> for AssociatedItemHirDefn {
    fn from(hir_defn: TraitAssociatedFnHirDefn) -> Self {
        AssociatedItemHirDefn::TraitItem(hir_defn.into())
    }
}

impl From<TraitAssociatedFnHirDefn> for HirDefn {
    fn from(hir_defn: TraitAssociatedFnHirDefn) -> Self {
        HirDefn::AssociatedItem(hir_defn.into())
    }
}

impl TraitAssociatedFnHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TraitItemPath,
        _hir_decl: TraitAssociatedFnHirDecl,
    ) -> TraitAssociatedFnHirDefn {
        todo!()
        // let syn_node_path = hir_decl.syn_node_path(db);
        // let mut parser = expr_parser(
        //     db,
        //     syn_node_path,
        //     Some(hir_decl.hir_expr_region(db)),
        //     AllowSelfType::True,
        //     AllowSelfValue::True,
        // );
        // let ast_idx = hir_decl.ast_idx(db);
        // let body = match parser.ast_sheet()[ast_idx] {
        //     Ast::Defn {
        //         block: DefnBlock::AssociatedItem { body },
        //         ..
        //     } => body.map(|body| parser.parse_block_expr(body)),
        //     _ => unreachable!(),
        // };
        // TraitForTypeMethodFnDefn::new(db, syn_node_path, hir_decl, body, parser.finish())
    }

    pub fn hir_eager_expr_region(self, db: &dyn HirDefnDb) -> Option<HirEagerExprRegion> {
        self.eager_body_with_hir_eager_expr_region(db)
            .map(|(_, region)| region)
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        trai_associated_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        trai_associated_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_associated_fn_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedFnHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).trai_path(db));
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
fn trai_associated_fn_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TraitAssociatedFnHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
