use super::*;
use husky_ast::Ast;
use salsa::DebugWithDb;

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMethodFnNodeDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub node_decl: TypeMethodFnNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeMethodFnNodeDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        node_path: TypeItemNodePath,
        node_decl: TypeMethodFnNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            node_path,
            node_decl.expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        Self::new_inner(db, node_path, node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = DefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMethodFnDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

impl TypeMethodFnDefn {
    pub(super) fn new(
        db: &dyn DefnDb,
        path: TypeItemPath,
        decl: TypeMethodFnDecl,
    ) -> DefnResult<Self> {
        let TypeItemNodeDefn::MethodFn(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        Ok(TypeMethodFnDefn::new_inner(
            db,
            path,
            decl,
            node_defn.body(db),
            node_defn.expr_region(db),
        ))
    }
}
