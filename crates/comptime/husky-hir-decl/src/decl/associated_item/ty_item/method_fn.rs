use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMethodFnHirDecl {
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    #[return_ref]
    pub self_parameter: EtherealTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: EtherealTerm,
}
