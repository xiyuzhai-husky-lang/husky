use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct FunctionDefn {
    #[id]
    pub path: FormPath,
    pub decl: FnDecl,
    pub expr_region: ExprRegion,
    pub body: DefnResult<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn function_defn(db: &dyn DefnDb, decl: FnDecl) -> FunctionDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::Entity(path.into()),
        Some(decl.expr_region(db)),
        AllowSelfType::False,
        AllowSelfValue::False,
    );
    let ast_idx = decl.ast_idx(db);
    let body = match parser.ast_sheet()[ast_idx] {
        Ast::Defn {
            block:
                DefnBlock::Form {
                    path,
                    body: Some(body),
                },
            ..
        } => parser
            .parse_block_expr(body)
            .ok_or(OriginalDefnError::ExpectBody.into()),
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    FunctionDefn::new(db, path, decl, expr_region, body)
}
