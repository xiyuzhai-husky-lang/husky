use super::*;
use husky_ast::Ast;
use salsa::DebugWithDb;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMethodFnSynNodeDefn {
    #[id]
    pub syn_node_path: TypeItemSynNodePath,
    pub syn_node_decl: TypeMethodFnSynNodeDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMethodFnSynNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        syn_node_path: TypeItemSynNodePath,
        syn_node_decl: TypeMethodFnSynNodeDecl,
    ) -> Self {
        let mut parser = expr_parser(
            db,
            syn_node_path,
            syn_node_decl.syn_expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::True,
        );
        let ast_idx = syn_node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        Self::new_inner(db, syn_node_path, syn_node_decl, body, parser.finish())
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TypeMethodFnSynDefn {
    #[id]
    pub path: TypeItemPath,
    pub decl: TypeMethodFnSynDecl,
    pub body: Option<SynExprIdx>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeMethodFnSynDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TypeItemPath,
        decl: TypeMethodFnSynDecl,
    ) -> SynDefnResult<Self> {
        let TypeItemSynNodeDefn::MethodFn(syn_node_defn) = path.syn_node_path(db).syn_node_defn(db)
        else {
            unreachable!()
        };
        Ok(TypeMethodFnSynDefn::new_inner(
            db,
            path,
            decl,
            syn_node_defn.body(db),
            syn_node_defn.syn_expr_region(db),
        ))
    }
}
