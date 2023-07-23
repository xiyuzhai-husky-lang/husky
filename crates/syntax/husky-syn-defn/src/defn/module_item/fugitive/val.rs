use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct ValSynNodeDefn {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    pub syn_node_decl: ValSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub expr_region: SynExprRegion,
}

impl ValSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: FugitiveSynNodePath,
        syn_node_decl: ValSynNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::Fugitive { body, .. },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        ValSynNodeDefn::new_inner(db, syn_node_path, syn_node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct ValSynDefn {
    #[id]
    pub path: FugitivePath,
    pub decl: ValSynDecl,
    pub body: Option<SynExprIdx>,
    pub expr_region: SynExprRegion,
}

impl ValSynDefn {
    pub(super) fn new(db: &dyn SynDefnDb, path: FugitivePath, decl: ValSynDecl) -> Self {
        let FugitiveSynNodeDefn::Val(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db) else {
            unreachable!()
        };
        Self::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.expr_region(db),
        )
    }
}
