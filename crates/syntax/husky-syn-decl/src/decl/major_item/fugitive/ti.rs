use super::*;

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAliasSynNodeDecl {
    #[id]
    pub syn_node_path: FugitiveSynNodePath,
    #[return_ref]
    pub template_parameter_obelisk_list:
        SynNodeDeclResult<Option<SynTemplateParameterSyndicateList>>,
    pub syn_expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDeclDb, jar = SynDeclJar)]
pub struct TypeAliasSynDecl {
    #[id]
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: TemplateSynParametersData,
    pub syn_expr_region: SynExprRegion,
}
