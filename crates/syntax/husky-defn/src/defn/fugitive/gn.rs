use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct GnDefn {
    #[id]
    pub node_id: FugitiveNodeId,
    pub decl: GnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl HasDefn for GnDecl {
    type Defn = GnDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        gn_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn gn_defn(db: &dyn DefnDb, decl: GnDecl) -> GnDefn {
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
    GnDefn::new(db, node_id, decl, body, parser.finish())
}
