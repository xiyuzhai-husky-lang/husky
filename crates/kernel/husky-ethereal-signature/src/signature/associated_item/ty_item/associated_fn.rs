use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypeAssociatedFnEtherealSignatureTemplate {
    path: TypeItemPath,
    self_ty: EtherealTerm,
    implicit_parameters: ImplicitParameterEtherealSignatures,
    regular_parameters: ExplicitParameterEtherealSignatures,
    return_ty: EtherealTerm,
}

impl TypeAssociatedFnEtherealSignatureTemplate {
    pub fn path(&self) -> TypeItemPath {
        self.path
    }

    pub fn self_ty(&self) -> EtherealTerm {
        self.self_ty
    }

    pub fn implicit_parameters(&self) -> &[ImplicitParameterEtherealSignature] {
        &self.implicit_parameters
    }

    pub fn regular_parameters(&self) -> &[ExplicitParameterEtherealSignature] {
        &self.regular_parameters
    }

    pub fn return_ty(&self) -> EtherealTerm {
        self.return_ty
    }
}
