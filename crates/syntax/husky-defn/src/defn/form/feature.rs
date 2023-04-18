use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct ValDefn {
    #[id]
    pub path: FormPath,
    pub decl: ValDecl,
    pub expr_region: ExprRegion,
    pub body: Option<ExprIdx>,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn feature_defn(db: &dyn DefnDb, decl: ValDecl) -> ValDefn {
    let path = decl.path(db);
    let mut parser = expr_parser(
        db,
        DefnRegionPath::Entity(path.into()),
        None,
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
    let expr_region = parser.finish();
    ValDefn::new(db, path, decl, expr_region, body)
}
