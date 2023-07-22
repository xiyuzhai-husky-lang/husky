use super::*;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct UnitStructTypeNodeDecl {
    #[id]
    pub syn_node_path: TypeSynNodePath,
    pub ast_idx: AstIdx,
    pub expr_region: SynExprRegion,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
}

impl UnitStructTypeNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct UnitStructTypeDecl {
    #[id]
    pub path: TypePath,
    pub expr_region: SynExprRegion,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
}

impl UnitStructTypeDecl {
    #[inline(always)]
    pub(super) fn from_node_decl(
        db: &dyn DeclDb,
        path: TypePath,
        syn_node_decl: UnitStructTypeNodeDecl,
    ) -> DeclResult<Self> {
        todo!()
    }
}
