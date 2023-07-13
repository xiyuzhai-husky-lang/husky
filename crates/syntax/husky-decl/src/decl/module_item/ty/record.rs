use super::*;

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RecordTypeNodeDecl {
    #[id]
    pub node_path: TypeNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<ImplicitParameterDeclList>>,
    pub expr_region: ExprRegion,
}

impl RecordTypeNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = DeclJar)]
pub struct RecordTypeDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub expr_region: ExprRegion,
}

impl RecordTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        node_decl: RecordTypeNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let expr_region = node_decl.expr_region(db);
        Ok(Self::new(db, path, generic_parameters, expr_region))
    }
}
