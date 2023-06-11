use super::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct GnDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: GnDecl,
    pub expr_region: ExprRegion,
    pub body: Option<ExprIdx>,
}

impl HasDefn for GnDecl {
    type Defn = GnDefn;

    fn defn(self, db: &dyn DefnDb) -> Self::Defn {
        gn_defn(db, self)
    }
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn gn_defn(db: &dyn DefnDb, decl: GnDecl) -> GnDefn {
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
            block: DefnBlock::Form { body, .. },
            ..
        } => body.map(|body| parser.parse_block_expr(body)),
        _ => unreachable!(),
    };
    let expr_region = parser.finish();
    GnDefn::new(db, path, decl, expr_region, body)
}
