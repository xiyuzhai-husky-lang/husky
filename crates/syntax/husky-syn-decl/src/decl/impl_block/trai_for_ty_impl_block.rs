use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = DeclDb, jar = SynDeclJar)]
pub struct TraitForTypeImplBlockNodeDecl {
    #[id]
    pub node_path: TraitForTypeImplBlockSynNodePath,
    pub ast_idx: AstIdx,
    pub impl_token: ImplToken,
    #[return_ref]
    implicit_parameter_decl_list: NodeDeclResult<Option<Generics>>,
    pub trai_expr: TraitExpr,
    pub for_token: ConnectionForToken,
    pub self_ty_decl: SelfTypeDecl,
    #[return_ref]
    pub eol_colon: NodeDeclResult<EolToken>,
    pub expr_region: SynExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelfTypeDecl {
    PathLeadingExpr(SelfTypeExpr),
    DeriveAny {
        at_token: AtToken,
        derive_token: DeriveToken,
        underscore_token: UnderscoreToken,
    },
}

impl TraitForTypeImplBlockNodeDecl {
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

impl HasNodeDecl for TraitForTypeImplBlockSynNodePath {
    type NodeDecl = TraitForTypeImplBlockNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        trai_for_ty_impl_block_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_impl_block_node_decl(
    db: &dyn DeclDb,
    node_path: TraitForTypeImplBlockSynNodePath,
) -> TraitForTypeImplBlockNodeDecl {
    let parser = DeclParser::new(db, node_path.module_path(db));
    parser.parse_trai_for_ty_impl_block_node_decl(node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_for_ty_impl_block_node_decl(
        &self,
        node_path: TraitForTypeImplBlockSynNodePath,
    ) -> TraitForTypeImplBlockNodeDecl {
        let db = self.db();
        let node = node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self
                .parse_trai_for_ty_impl_block_node_decl_aux(
                    node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                )
                .into(),
            _ => unreachable!(),
        }
    }

    fn parse_trai_for_ty_impl_block_node_decl_aux(
        &self,
        node_path: TraitForTypeImplBlockSynNodePath,
        node: TraitForTypeImplBlockSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> TraitForTypeImplBlockNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            node.node_path(db),
            None,
            AllowSelfType::True,
            AllowSelfValue::False,
        );
        let mut ctx = parser.ctx(None, token_group_idx, None);
        let impl_token = ctx.try_parse_option().unwrap().unwrap();
        let implicit_parameter_decl_list = ctx.try_parse_option();
        // ad hoc
        let trai: TraitExpr = ctx.try_parse_option().unwrap().unwrap();
        let for_token = ctx
            .try_parse_option()
            .expect("guaranteed by parsing")
            .expect("guaranteed by parsing");
        let ty = match node.ty_sketch_expr(db) {
            SelfTypeSketchExpr::Path(_) => SelfTypeDecl::PathLeadingExpr(
                ctx.try_parse_option()
                    .expect("guaranteed")
                    .expect("guaranteed"),
            ),
            SelfTypeSketchExpr::DeriveAny {
                at_token,
                derive_token,
                underscore_token,
            } => {
                ctx.advance_by(3);
                SelfTypeDecl::DeriveAny {
                    at_token,
                    derive_token,
                    underscore_token,
                }
            }
        };
        let eol_colon = ctx.try_parse_expected(OriginalNodeDeclError::ExpectedEolColon);
        TraitForTypeImplBlockNodeDecl::new(
            db,
            node_path,
            ast_idx,
            impl_token,
            implicit_parameter_decl_list,
            trai,
            for_token,
            ty,
            eol_colon,
            parser.finish(),
        )
    }
}

#[salsa::tracked(db = DeclDb, jar = SynDeclJar, constructor = new)]
pub struct TraitForTypeImplBlockDecl {
    #[id]
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub trai_expr: TraitExpr,
    pub self_ty_decl: SelfTypeDecl,
    pub expr_region: SynExprRegion,
}

impl HasDecl for TraitForTypeImplBlockPath {
    type Decl = TraitForTypeImplBlockDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        trai_for_ty_impl_block_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_impl_block_decl(
    db: &dyn DeclDb,
    path: TraitForTypeImplBlockPath,
) -> DeclResult<TraitForTypeImplBlockDecl> {
    let node_decl = path.node_path(db).node_decl(db);
    TraitForTypeImplBlockDecl::from_node_decl(db, path, node_decl)
}

impl TraitForTypeImplBlockDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeImplBlockPath,
        node_decl: TraitForTypeImplBlockNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let trai_expr = node_decl.trai_expr(db);
        let self_ty_decl = node_decl.self_ty_decl(db);
        let expr_region = node_decl.expr_region(db);
        node_decl.eol_colon(db).as_ref()?;
        Ok(Self::new(
            db,
            path,
            generic_parameters,
            trai_expr,
            self_ty_decl,
            expr_region,
        ))
    }
}
