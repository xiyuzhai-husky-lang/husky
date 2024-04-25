use super::*;
use husky_hir_decl::decl::TraitAssocRitchieHirDecl;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TraitAssocRitchieHirDefn {
    pub path: TraitItemPath,
    pub hir_decl: TraitAssocRitchieHirDecl,
    pub eager_body_with_hir_eager_expr_region: Option<(HirEagerExprIdx, HirEagerExprRegion)>,
}

impl From<TraitAssocRitchieHirDefn> for AssocItemHirDefn {
    fn from(hir_defn: TraitAssocRitchieHirDefn) -> Self {
        AssocItemHirDefn::TraitItem(hir_defn.into())
    }
}

impl From<TraitAssocRitchieHirDefn> for HirDefn {
    fn from(hir_defn: TraitAssocRitchieHirDefn) -> Self {
        HirDefn::AssocItem(hir_defn.into())
    }
}

impl TraitAssocRitchieHirDefn {
    pub(super) fn new(
        _db: &::salsa::Db,
        _path: TraitItemPath,
        _hir_decl: TraitAssocRitchieHirDecl,
    ) -> TraitAssocRitchieHirDefn {
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
        //         block: DefnBlock::AssocItem { body },
        //         ..
        //     } => body.map(|body| parser.parse_block_expr(body)),
        //     _ => unreachable!(),
        // };
        // TraitForTypeMethodRitchieDefn::new(db, syn_node_path, hir_decl, body, parser.finish())
    }

    pub fn hir_eager_expr_region(self, db: &::salsa::Db) -> Option<HirEagerExprRegion> {
        self.eager_body_with_hir_eager_expr_region(db)
            .map(|(_, region)| region)
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        trai_assoc_fn_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        trai_assoc_fn_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn trai_assoc_fn_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TraitAssocRitchieHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_item_path(hir_decl.path(db).trai_path(db));
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

#[salsa::tracked(jar = HirDefnJar)]
fn trai_assoc_fn_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TraitAssocRitchieHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
