use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedFnDefn {
    #[id]
    pub id: AssociatedItemId,
    pub decl: TypeAssociatedFnDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_fn_defn(
    db: &dyn DefnDb,
    decl: TypeAssociatedFnDecl,
) -> TypeAssociatedFnDefn {
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
        Ast::Defn { children, .. } => parser
            .parse_block_expr(children.form_body().expect("todo: deal with form variants"))
            .ok_or(OriginalDefnError::ExpectBody.into()), // todo: change this to parse expected
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    TypeAssociatedFnDefn::new(db, id, decl, expr_region, body)
}
