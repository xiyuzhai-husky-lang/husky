use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TraitForTypeImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeImplBlockSynNodePath,
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

impl TraitForTypeImplBlockSynNodeDecl {
    pub fn errors(self, db: &dyn SynDeclDb) -> NodeDeclErrorRefs {
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
    type NodeDecl = TraitForTypeImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        trai_for_ty_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_impl_block_syn_node_decl(
    db: &dyn SynDeclDb,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> TraitForTypeImplBlockSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_trai_for_ty_impl_block_syn_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_for_ty_impl_block_syn_node_decl(
        &self,
        syn_node_path: TraitForTypeImplBlockSynNodePath,
    ) -> TraitForTypeImplBlockSynNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::ImplBlock {
                token_group_idx,
                items: _,
            } => self
                .parse_trai_for_ty_impl_block_syn_node_decl_aux(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                )
                .into(),
            _ => unreachable!(),
        }
    }

    fn parse_trai_for_ty_impl_block_syn_node_decl_aux(
        &self,
        syn_node_path: TraitForTypeImplBlockSynNodePath,
        node: TraitForTypeImplBlockSynNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
    ) -> TraitForTypeImplBlockSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(
            node.syn_node_path(db),
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
        TraitForTypeImplBlockSynNodeDecl::new(
            db,
            syn_node_path,
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

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct TraitForTypeImplBlockSynDecl {
    #[id]
    pub path: TraitForTypeImplBlockPath,
    #[return_ref]
    pub generic_parameters: ImplicitParameterDeclPatterns,
    pub trai_expr: TraitExpr,
    pub self_ty_decl: SelfTypeDecl,
    pub expr_region: SynExprRegion,
}

impl HasSynDecl for TraitForTypeImplBlockPath {
    type Decl = TraitForTypeImplBlockSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        trai_for_ty_impl_block_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_impl_block_syn_decl(
    db: &dyn SynDeclDb,
    path: TraitForTypeImplBlockPath,
) -> DeclResult<TraitForTypeImplBlockSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitForTypeImplBlockSynDecl::from_node_decl(db, path, syn_node_decl)
}

impl TraitForTypeImplBlockSynDecl {
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: TraitForTypeImplBlockPath,
        syn_node_decl: TraitForTypeImplBlockSynNodeDecl,
    ) -> DeclResult<Self> {
        let generic_parameters = syn_node_decl
            .implicit_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.generic_parameters().to_smallvec())
            .unwrap_or_default();
        let trai_expr = syn_node_decl.trai_expr(db);
        let self_ty_decl = syn_node_decl.self_ty_decl(db);
        let expr_region = syn_node_decl.expr_region(db);
        syn_node_decl.eol_colon(db).as_ref()?;
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
