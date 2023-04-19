use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeMemoizedFieldDefn {
    #[id]
    pub path: Option<TypeItemPath>,
    pub decl: TypeMemoizedFieldDecl,
    pub expr_region: ExprRegion,
    pub body: Option<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_memo_defn(db: &dyn DefnDb, decl: TypeMemoizedFieldDecl) -> TypeMemoizedFieldDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::AssociatedItem(decl.associated_item(db).id(db)),
        Some(decl.expr_region(db)),
        AllowSelfType::True,
        AllowSelfValue::True,
    );
    let ast_idx = decl.ast_idx(db);
    // todo: deal with no body case
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn {
            block: DefnBlock::AssociatedItem { body },
            ..
        } => body.map(|body| parser.parse_block_expr(body)),
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TypeMemoizedFieldDefn::new(db, path, decl, expr_region, body)
}
