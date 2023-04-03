use crate::*;
use husky_ast::Ast;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeMethodFnDefn {
    #[id]
    pub id: AssociatedItemId,
    pub decl: TypeMethodFnDecl,
    pub expr_region: ExprRegion,
    pub body: Option<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_method_fn_defn(db: &dyn DefnDb, decl: TypeMethodFnDecl) -> TypeMethodFnDefn {
    let id = decl.id(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(id),
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
    TypeMethodFnDefn::new(db, id, decl, expr_region, body)
}
