use super::*;
use husky_entity_path::path::impl_block::{
    trai_for_ty_impl_block::TraitForTypeImplBlockPath, TypeSketch,
};
use husky_entity_tree::node::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockSynNodePath;

#[salsa::tracked]
pub struct TraitForTypeImplBlockSynNodeDecl {
    #[id]
    pub syn_node_path: TraitForTypeImplBlockSynNodePath,
    pub impl_regional_token: ImplRegionalToken,
    #[return_ref]
    template_parameter_decl_list: SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub trai_expr: TraitSyndicate,
    pub for_token: ConnectionForRegionalToken,
    pub self_ty_decl: SelfTypeDecl,
    #[return_ref]
    pub eol_colon: SynNodeDeclResult<EolRegionalToken>,
    pub syn_expr_region: SynExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SelfTypeDecl {
    PathLeadingExpr(SelfTypeSyndicate),
    DeriveAny {
        pound_token: PoundRegionalToken,
        derive_token: DeriveRegionalToken,
        underscore_token: UnderscoreRegionalToken,
    },
}

impl TraitForTypeImplBlockSynNodeDecl {
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

impl HasSynNodeDecl for TraitForTypeImplBlockSynNodePath {
    type NodeDecl = TraitForTypeImplBlockSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a ::salsa::Db) -> Self::NodeDecl {
        trai_for_ty_impl_block_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_impl_block_syn_node_decl(
    db: &::salsa::Db,
    syn_node_path: TraitForTypeImplBlockSynNodePath,
) -> TraitForTypeImplBlockSynNodeDecl {
    let parser = ItemSynNodeDeclParser::new(db, syn_node_path.into());
    parser.parse_trai_for_ty_impl_block_syn_node_decl(syn_node_path)
}

impl<'a> ItemSynNodeDeclParser<'a> {
    fn parse_trai_for_ty_impl_block_syn_node_decl(
        &self,
        syn_node_path: TraitForTypeImplBlockSynNodePath,
    ) -> TraitForTypeImplBlockSynNodeDecl {
        let db = self.db();
        let mut parser = self.expr_parser(None, AllowSelfType::True, AllowSelfValue::False, None);
        let impl_token = parser.try_parse_option().unwrap().unwrap();
        let template_parameter_decl_list = parser.try_parse_option();
        // ad hoc
        let trai: TraitSyndicate = parser.try_parse_option().unwrap().unwrap();
        let for_token = parser
            .try_parse_option()
            .expect("guaranteed by parsing")
            .expect("guaranteed by parsing");

        let ty = match syn_node_path.ty_sketch(db) {
            TypeSketch::DeriveAny => SelfTypeDecl::DeriveAny {
                pound_token: parser
                    .try_parse_option()
                    .expect("guaranteed")
                    .expect("guaranteed"),
                derive_token: parser
                    .try_parse_option()
                    .expect("guaranteed")
                    .expect("guaranteed"),
                underscore_token: parser
                    .try_parse_option()
                    .expect("guaranteed")
                    .expect("guaranteed"),
            },
            TypeSketch::Path(_) => SelfTypeDecl::PathLeadingExpr(
                parser
                    .try_parse_option()
                    .expect("guaranteed")
                    .expect("guaranteed"),
            ),
        };
        let eol_colon = parser.try_parse_expected(OriginalSynNodeDeclError::ExpectedEolColon);
        TraitForTypeImplBlockSynNodeDecl::new(
            db,
            syn_node_path,
            impl_token,
            template_parameter_decl_list,
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
    pub template_parameters: TemplateSynParametersData,
    pub trai_expr: TraitSyndicate,
    pub self_ty_decl: SelfTypeDecl,
    pub syn_expr_region: SynExprRegion,
}

impl HasSynDecl for TraitForTypeImplBlockPath {
    type Decl = TraitForTypeImplBlockSynDecl;

    fn syn_decl(self, db: &::salsa::Db) -> SynDeclResult<Self::Decl> {
        trai_for_ty_impl_block_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_impl_block_syn_decl(
    db: &::salsa::Db,
    path: TraitForTypeImplBlockPath,
) -> SynDeclResult<TraitForTypeImplBlockSynDecl> {
    let syn_node_decl = path.syn_node_path(db).syn_node_decl(db);
    TraitForTypeImplBlockSynDecl::from_node(db, path, syn_node_decl)
}

impl TraitForTypeImplBlockSynDecl {
    fn from_node(
        db: &::salsa::Db,
        path: TraitForTypeImplBlockPath,
        syn_node_decl: TraitForTypeImplBlockSynNodeDecl,
    ) -> SynDeclResult<Self> {
        let template_parameters = syn_node_decl
            .template_parameter_decl_list(db)
            .as_ref()?
            .as_ref()
            .map(|list| {
                list.syn_template_parameter_obelisks()
                    .iter()
                    .map(Clone::clone)
                    .collect()
            })
            .unwrap_or_default();
        let trai_expr = syn_node_decl.trai_expr(db);
        let self_ty_decl = syn_node_decl.self_ty_decl(db);
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        syn_node_decl.eol_colon(db).as_ref()?;
        Ok(Self::new(
            db,
            path,
            template_parameters,
            trai_expr,
            self_ty_decl,
            syn_expr_region,
        ))
    }
}
