use super::*;
use husky_term_prelude::RitchieTypeKind;

#[salsa::tracked(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedFnEthTemplate {
    #[id]
    pub path: TypeItemPath,
    // todo: is this necessary?
    pub self_ty: EthTerm,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealParenateParameters,
    pub return_ty: EthTerm,
    pub ty: EthTerm,
}

impl TypeAssociatedFnEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypeItemPath,
        declarative_signature: TypeAssociatedFnDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EthTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
        let template_parameters = EthTemplateParameters::from_declarative(
            db,
            declarative_signature.template_parameters(db),
        )?;
        let parenate_parameters = EtherealParenateParameters::from_declarative(
            db,
            declarative_signature.parenate_parameters(db),
        )?;
        let return_ty = EthTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        let ty = RitchieEthTerm::new(
            db,
            RitchieTypeKind::Fn.into(),
            parenate_parameters.iter().copied(),
            return_ty,
        )?
        .into();
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            parenate_parameters,
            return_ty,
            ty,
        ))
    }
}
