use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new_inner)]
pub struct TaskAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub task_token: IdentRegionalToken,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

/// # constructor

impl TaskAttrSynNodeDecl {
    pub(super) fn new(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> Self {
        let parser_factory = ItemDeclParser::new(db, syn_node_path.into());
        let mut parser = parser_factory.expr_parser(
            syn_node_path
                .parent_syn_node_path(db)
                .syn_node_decl(db)
                .syn_expr_region(db),
            AllowSelfType::False,
            AllowSelfValue::False,
            None,
        );
        let pound_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let task_token = parser
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        Self::new_inner(db, syn_node_path, pound_token, task_token, parser.finish())
    }
}

/// # getters

impl TaskAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked]
pub struct TaskAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    // todo: Tasks
    pub syn_expr_region: SynExprRegion,
}

impl TaskAttrSynDecl {
    pub(super) fn from_node(
        path: AttrItemPath,
        syn_node_decl: TaskAttrSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
