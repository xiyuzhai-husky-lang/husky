use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeMethodFnEtherealSignatureTemplate {
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub implicit_parameters: ImplicitParameterEtherealSignatures,
    #[return_ref]
    pub self_parameter: ExplicitParameterEtherealSignature,
    #[return_ref]
    pub explicit_parameters: ExplicitParameterEtherealSignatures,
    pub return_ty: EtherealTerm,
}

impl TypeMethodFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        declarative_signature: TypeMethodFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
        let implicit_parameters = ImplicitParameterEtherealSignatures::from_declarative(
            db,
            declarative_signature.implicit_parameters(db),
        )?;
        let self_parameter = ExplicitParameterEtherealSignature::from_declarative(
            db,
            declarative_signature.self_parameter(db),
        )?;
        let explicit_parameters = ExplicitParameterEtherealSignatures::from_declarative(
            db,
            declarative_signature.explicit_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        Ok(Self::new(
            db,
            self_ty,
            implicit_parameters,
            self_parameter,
            explicit_parameters,
            return_ty,
        ))
    }
}
