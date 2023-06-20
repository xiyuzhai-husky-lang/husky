use super::*;
use husky_ast::Ast;
use salsa::DebugWithDb;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeMethodFnDefn {
    #[id]
    pub node_path: TypeItemNodePath,
    pub decl: TypeMethodFnDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: ExprRegion,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_method_fn_defn(db: &dyn DefnDb, decl: TypeMethodFnDecl) -> TypeMethodFnDefn {
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
    // TypeMethodFnDefn::new(db, node_path, decl, body, parser.finish())
}
