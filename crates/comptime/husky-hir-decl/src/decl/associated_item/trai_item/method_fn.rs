use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TraitMethodFnHirDecl {
    pub path: TraitItemPath,
    pub generic_parameters: EtherealGenericParameters,
    /// `Self` as generic parameter
    pub self_ty_generic_parameter: EtherealGenericParameter,
    pub hir_expr_region: HirEagerExprRegion,
}
