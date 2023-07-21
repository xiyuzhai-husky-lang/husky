use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeNodeDefn {
    #[id]
    pub node_path: TraitForTypeItemSynNodePath,
    pub node_decl: TraitForTypeAssociatedTypeNodeDecl,
    pub body: Option<ExprIdx>,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedTypeNodeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        node_path: TraitForTypeItemSynNodePath,
        node_decl: TraitForTypeAssociatedTypeNodeDecl,
    ) -> Self {
        let node_path = node_decl.node_path(db);
        let mut parser = expr_parser(
            db,
            node_path,
            node_decl.expr_region(db),
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let ast_idx = node_decl.ast_idx(db);
        let body = match parser.ast_sheet()[ast_idx] {
            Ast::Defn {
                block: DefnBlock::AssociatedItem { body },
                ..
            } => body.map(|body| parser.parse_block_expr(body)),
            _ => unreachable!(),
        };
        TraitForTypeAssociatedTypeNodeDefn::new_inner(
            db,
            node_path,
            node_decl,
            body,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitForTypeAssociatedTypeDefn {
    #[id]
    pub path: TraitForTypeItemPath,
    pub decl: TraitForTypeAssociatedTypeDecl,
    pub expr_region: SynExprRegion,
}

impl TraitForTypeAssociatedTypeDefn {
    pub(super) fn new(
        db: &dyn SynDefnDb,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssociatedTypeDecl,
    ) -> Self {
        let TraitForTypeItemSynNodeDefn::AssociatedType(node_defn) = path.node_path(db).node_defn(db) else {
            unreachable!()
        };
        TraitForTypeAssociatedTypeDefn::new_inner(db, path, decl, node_defn.expr_region(db))
    }
}
