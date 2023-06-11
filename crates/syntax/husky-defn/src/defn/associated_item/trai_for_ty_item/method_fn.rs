use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeMethodFnDefn {
    #[id]
    pub path: Option<TraitForTypeItemPath>,
    pub decl: TraitForTypeMethodFnDecl,
    pub expr_region: ExprRegion,
    pub body: Option<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_method_defn(
    db: &dyn DefnDb,
    decl: TraitForTypeMethodFnDecl,
) -> TraitForTypeMethodFnDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(decl.associated_item(db).id(db)),
        Some(decl.expr_region(db)),
        AllowSelfType::True,
        AllowSelfValue::True,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn {
            block: DefnBlock::AssociatedItem { body },
            ..
        } => body.map(|body| parser.parse_block_expr(body)),
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TraitForTypeMethodFnDefn::new(db, Some(path), decl, expr_region, body)
}
