use super::*;
use husky_token::EolToken;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TypeImplBlockNodeDecl {
    #[id]
    pub node_path: TypeImplBlockSynNodePath,
    pub ast_idx: AstIdx,
    pub impl_block: TypeImplBlockSynNode,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub self_ty_expr: SelfTypeExpr,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
    pub expr_region: SynExprRegion,
}

impl TypeImplBlockNodeDecl {
    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        SmallVec::from_iter(
            self.implicit_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

impl HasNodeDecl for TypeImplBlockSynNode {
    type NodeDecl = TypeImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        self.node_path(db).node_decl(db)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_impl_block_node_decl(
    db: &dyn DeclDb,
    node_path: TypeImplBlockSynNodePath,
) -> TypeImplBlockNodeDecl {
    let parser = DeclParser::new(db, node_path.module_path(db));
    parser.parse_ty_impl_block_node_decl(node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_ty_impl_block_node_decl(
        &self,
        node_path: TypeImplBlockSynNodePath,
    ) -> TypeImplBlockNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self.parse_ty_impl_block_decl_aux(node_path, node, ast_idx, token_group_idx),
            _ => unreachable!(),
        }
    }

    fn parse_ty_impl_block_decl_aux(
        &self,
        node_path: TypeImplBlockSynNodePath,
        node: TypeImplBlockSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> TypeImplBlockNodeDecl {
        let db = self.db();
        let mut parser =
            self.expr_parser(node_path, None, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = parser.ctx(None, token_group_idx, None);
        let impl_token = ctx.try_parse_option().unwrap().unwrap();
        let implicit_parameter_decl_list = ctx.try_parse_option();
        let ty = ctx.try_parse_option().unwrap().unwrap();
        let eol_colon = ctx.try_parse_expected(OriginalNodeDeclError::ExpectedEolColon);
        TypeImplBlockNodeDecl::new(
            db,
            node_path,
            ast_idx,
            node,
            impl_token,
            implicit_parameter_decl_list,
            ty,
            eol_colon,
            parser.finish(),
        )
    }
}

impl HasNodeDecl for TypeImplBlockSynNodePath {
    type NodeDecl = TypeImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        ty_impl_block_node_decl(db, self)
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar, constructor = new)]
pub struct TypeImplBlockDecl {
    #[id]
    pub path: TypeImplBlockPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub self_ty_expr: SelfTypeExpr,
    pub expr_region: SynExprRegion,
}

impl From<TypeImplBlockDecl> for Decl {
    fn from(decl: TypeImplBlockDecl) -> Self {
        Decl::ImplBlock(decl.into())
    }
}

impl TypeImplBlockDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TypeImplBlockPath,
        node_decl: TypeImplBlockNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let self_ty_expr = node_decl.self_ty_expr(db);
        let expr_region = node_decl.expr_region(db);
        node_decl.eol_colon(db).as_ref()?;
        Ok(Self::new(
            db,
            path,
            generic_parameters,
            self_ty_expr,
            expr_region,
        ))
    }
}

impl HasDecl for TypeImplBlockPath {
    type Decl = TypeImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        ty_impl_block_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_impl_block_decl(
    db: &dyn DeclDb,
    // here use path instead of node_path because salsa doesn't support use wrapper type by default
    // maybe add AsId carefully
    path: TypeImplBlockPath,
) -> DeclResult<TypeImplBlockDecl> {
    let node_path = path.node_path(db);
    let node_decl = node_path.node_decl(db);
    TypeImplBlockDecl::from_node_decl(db, path, node_decl)
}
