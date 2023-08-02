use super::*;
use husky_term_prelude::RitchieKind;

#[salsa::tracked(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedFnEtherealSignatureTemplate {
    #[id]
    pub path: TypeItemPath,
    // todo: is this necessary?
    pub self_ty: EtherealTerm,
    #[return_ref]
    pub template_parameters: EtherealTermTemplateParameters,
    #[return_ref]
    pub parenate_parameters: EtherealTermParenateParameters,
    pub return_ty: EtherealTerm,
    pub ty: EtherealTerm,
}

impl TypeAssociatedFnEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &dyn EtherealSignatureDb,
        path: TypeItemPath,
        declarative_signature: TypeAssociatedFnDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let self_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.self_ty(db))?;
        let template_parameters = EtherealTermTemplateParameters::from_declarative(
            db,
            declarative_signature.template_parameters(db),
        )?;
        let parenate_parameters = EtherealTermParenateParameters::from_declarative(
            db,
            declarative_signature.parenate_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        let ty = EtherealTermRitchie::new(
            db,
            RitchieKind::FnType,
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
