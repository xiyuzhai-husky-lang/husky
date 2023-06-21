use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct GnNodeDefn {
    #[id]
    pub node_path: FugitiveNodePath,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct GnDefn {
    #[id]
    pub node_path: FugitiveNodePath,
    pub decl: GnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn gn_defn(db: &dyn DefnDb, decl: GnDecl) -> GnDefn {
    todo!()
    // let node_path = decl.node_path(db);
    // let mut parser = expr_parser(
    //     db,
    //     node_path,
    //     Some(decl.expr_region(db)),
    //     AllowSelfType::False,
    //     AllowSelfValue::False,
    // );
    // let ast_idx = decl.ast_idx(db);
    // let body = match parser.ast_sheet()[ast_idx] {
    //     Ast::Defn {
    //         block: DefnBlock::Form { body, .. },
    //         ..
    //     } => body.map(|body| parser.parse_block_expr(body)),
    //     _ => unreachable!(),
    // };
    // GnDefn::new(db, node_path, decl, body, parser.finish())
}
