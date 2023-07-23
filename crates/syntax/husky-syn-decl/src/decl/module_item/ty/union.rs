use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnionTypeSynNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub syn_expr_region: SynExprRegion,
}

impl UnionTypeSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct UnionTypeSynDecl {
    #[id]
    pub path: TypePath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub syn_expr_region: SynExprRegion,
}

impl UnionTypeSynDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypePath,
        syn_node_decl: UnionTypeSynNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
