use super::*;

#[salsa::tracked(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedFnEtherealSignatureTemplate {
    #[id]
    pub path: TypeItemPath,
    pub self_ty: EtherealTerm,
    pub implicit_parameters: ImplicitParameterEtherealSignatures,
    pub regular_parameters: ExplicitParameterEtherealSignatures,
    pub return_ty: EtherealTerm,
}

impl TypeAssociatedFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeItemPath,
        declarative_signature: TypeAssociatedFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
        let implicit_parameters = ImplicitParameterEtherealSignatures::from_declarative(
            db,
            declarative_signature.implicit_parameters(db),
        )?;
        let regular_parameters = ExplicitParameterEtherealSignatures::from_declarative(
            db,
            declarative_signature.regular_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        Ok(Self::new(
            db,
            path,
            self_ty,
            implicit_parameters,
            regular_parameters,
            return_ty,
        ))
    }
}
