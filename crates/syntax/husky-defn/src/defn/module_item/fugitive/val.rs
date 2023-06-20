use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ValDefn {
    #[id]
    pub node_path: FugitiveNodePath,
    pub decl: ValDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl HasDefn for ValDecl {
    type Defn = ValDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        val_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn val_defn(db: &dyn DefnDb, decl: ValDecl) -> ValDefn {
    todo!()
    // let node_path = decl.node_path(db);
    // let mut parser = expr_parser(
    //     db,
    //     node_path,
    //     None,
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
    // ValDefn::new(db, node_path, decl, body, parser.finish())
}
