use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct FnDefn {
    #[id]
    pub node_id: FugitiveNodeId,
    pub decl: FnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl HasDefn for FnDecl {
    type Defn = FnDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        fn_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn fn_defn(db: &dyn DefnDb, decl: FnDecl) -> FnDefn {
    let node_id = decl.node_id(db);
    let mut parser = expr_parser(
        db,
        node_id,
        Some(decl.expr_region(db)),
        AllowSelfType::False,
        AllowSelfValue::False,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn {
            block: DefnBlock::Form { body, .. },
            ..
        } => body.map(|body| parser.parse_block_expr(body)),
        _ => unreachable!(),
    };
    FnDefn::new(db, node_id, decl, body, parser.finish())
}
