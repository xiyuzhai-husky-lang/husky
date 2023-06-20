use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TraitForTypeMethodFnDefn {
    #[id]
    pub node_path: TraitForTypeItemNodePath,
    pub decl: TraitForTypeMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn trai_for_ty_method_defn(
    db: &dyn DefnDb,
    decl: TraitForTypeMethodFnDecl,
) -> TraitForTypeMethodFnDefn {
    todo!()
    // let node_path = decl.node_path(db);
    // let mut parser = expr_parser(
    //     db,
    //     node_path,
    //     Some(decl.expr_region(db)),
    //     AllowSelfType::True,
    //     AllowSelfValue::True,
    // );
    // let ast_idx = decl.ast_idx(db);
    // let body = match parser.ast_sheet()[ast_idx] {
    //     Ast::Defn {
    //         block: DefnBlock::AssociatedItem { body },
    //         ..
    //     } => body.map(|body| parser.parse_block_expr(body)),
    //     _ => unreachable!(),
    // };
    // TraitForTypeMethodFnDefn::new(db, node_path, decl, body, parser.finish())
}
