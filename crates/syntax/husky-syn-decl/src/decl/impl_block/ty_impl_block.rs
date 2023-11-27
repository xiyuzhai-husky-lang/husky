use super::*;
use husky_regional_token::EolRegionalToken;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: TypeImplBlockSynNodePath,
    pub impl_regional_token: ImplRegionalToken,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub self_ty_expr: SelfTypeSyndicate,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

impl TypeImplBlockSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        SmallVec::from_iter(
            self.template_parameter_decl_list(db)
                .as_ref()
                .err()
                .into_iter()
                .chain(self.eol_colon(db).as_ref().err().into_iter()),
        )
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_impl_block_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TypeImplBlockSynNodePath,
) -> TypeImplBlockSynNodeDecl {
    let parser = DeclParser::new(db, syn_node_path);
    parser.parse_ty_impl_block_syn_node_decl()
}

impl<'a> DeclParser<'a> {
    fn parse_ty_impl_block_syn_node_decl(&self) -> TypeImplBlockSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let impl_token = parser.try_parse_option().unwrap().unwrap();
        let template_parameter_decl_list = parser.try_parse_option();
        let ty = parser.try_parse_option().unwrap().unwrap();
        let eol_colon = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEolColon);
        TypeImplBlockSynNodeDecl::new(
            db,
            self.syn_node_path(),
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

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        ty_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar, constructor = new)]
pub struct TypeImplBlockSynDecl {
    #[id]
    pub path: TypeImplBlockPath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub self_ty_expr: SelfTypeSyndicate,
    pub syn_expr_region: SynExprRegion,
}

impl From<TypeImplBlockSynDecl> for SynDecl {
    fn from(decl: TypeImplBlockSynDecl) -> Self {
        SynDecl::ImplBlock(decl.into())
    }
}

impl TypeImplBlockSynDecl {
    fn from_node_decl(
        db: &::salsa::Db,
        path: TypeImplBlockPath,
        syn_node_decl: TypeImplBlockSynNodeDecl,
    ) -> DeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| list.syn_template_parameter_obelisks().to_smallvec())
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

    fn syn_decl(self, db: &::salsa::Db) -> DeclResult<Self::Decl> {
        ty_impl_block_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn ty_impl_block_syn_decl(
    db: &::salsa::Db,
    // here use path instead of syn_node_path because salsa doesn't support use wrapper type by default
    // maybe add AsId carefully
    path: TypeImplBlockPath,
) -> DeclResult<TypeImplBlockSynDecl> {
    let syn_node_path = path.syn_node_path(db);
    let syn_node_decl = syn_node_path.syn_node_decl(db);
    TypeImplBlockSynDecl::from_node_decl(db, path, syn_node_decl)
}
