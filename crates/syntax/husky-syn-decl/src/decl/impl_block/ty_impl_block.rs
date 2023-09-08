use super::*;
use husky_token::EolToken;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: TypeImplBlockSynNodePath,
    pub ast_idx: AstIdx,
    pub impl_block: TypeImplBlockSynNode,
    pub impl_token: ImplToken,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<Generics>>,
    pub self_ty_expr: SelfTypeObelisk,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeImplBlockSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl HasSynNodeDecl for TypeImplBlockSynNode {
    type NodeDecl = TypeImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        self.syn_node_path(db).syn_node_decl(db)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_impl_block_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TypeImplBlockSynNodePath,
) -> TypeImplBlockSynNodeDecl {
    let parser = DeclParserFactory::new(db, syn_node_path);
    parser.parse_ty_impl_block_syn_node_decl(syn_node_path)
}

impl<'a> DeclParserFactory<'a> {
    fn parse_ty_impl_block_syn_node_decl(
        &self,
        syn_node_path: TypeImplBlockSynNodePath,
    ) -> TypeImplBlockSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => {
                self.parse_ty_impl_block_syn_decl_aux(syn_node_path, node, ast_idx, token_group_idx)
            }
            _ => unreachable!(),
        }
    }

    fn parse_ty_impl_block_syn_decl_aux(
        &self,
        syn_node_path: TypeImplBlockSynNodePath,
        node: TypeImplBlockSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> TypeImplBlockSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            syn_node_path,
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, None);
        let impl_token = ctx.try_parse_option().unwrap().unwrap();
        let template_parameter_decl_list = ctx.try_parse_option();
        let ty = ctx.try_parse_option().unwrap().unwrap();
        let eol_colon = ctx.try_parse_expected(OriginalSynNodeDeclError::ExpectedEolColon);
        TypeImplBlockSynNodeDecl::new(
            db,
            syn_node_path,
            ast_idx,
            node,
            impl_token,
            template_parameter_decl_list,
            ty,
            eol_colon,
            parser.finish(),
        )
    }
}

impl HasSynNodeDecl for TypeImplBlockSynNodePath {
    type NodeDecl = TypeImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        ty_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct TypeImplBlockSynDecl {
    #[id]
    pub path: TypeImplBlockPath,
    #[return_ref]
    pub template_parameters: ImplicitParameterDeclPatterns,
    pub self_ty_expr: SelfTypeObelisk,
    pub syn_expr_region: SynExprRegion,
}

impl From<TypeImplBlockSynDecl> for SynDecl {
    fn from(decl: TypeImplBlockSynDecl) -> Self {
        SynDecl::ImplBlock(decl.into())
    }
}

impl TypeImplBlockSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TypeImplBlockPath,
        syn_node_decl: TypeImplBlockSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.template_parameters().to_smallvec())
            .unwrap_or_default();
        let self_ty_expr = syn_node_decl.self_ty_expr(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        syn_node_decl.eol_colon(db).as_ref()?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            self_ty_expr,
            syn_expr_region,
        ))
    }
}

impl HasSynDecl for TypeImplBlockPath {
    type Decl = TypeImplBlockSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        ty_impl_block_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_impl_block_syn_decl(
    db: &dyn SynDeclDb,
    // here use path instead of syn_node_path because salsa doesn't support use wrapper type by default
    // maybe add AsId carefully
    path: TypeImplBlockPath,
) -> DeclResult<TypeImplBlockSynDecl> {
    let syn_node_path = path.syn_node_path(db);
    let syn_node_decl = syn_node_path.syn_node_decl(db);
    TypeImplBlockSynDecl::from_node_decl(db, path, syn_node_decl)
}
