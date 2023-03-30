use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeMemoDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMemoDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_memo_defn(db: &dyn DefnDb, decl: TypeMemoDecl) -> TypeMemoDefn {
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
        Ast::Defn { body, .. } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TypeMemoDefn::new(db, path, decl, expr_region, body)
}
