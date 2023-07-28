use super::*;
use husky_term_prelude::RitchieKind;

#[salsa::tracked(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct TypeAssociatedFnEtherealSignatureTemplate {
    #[id]
    pub path: TypeItemPath,
    pub self_ty: EtherealTerm,
    pub template_parameters: EtherealTemplateParameters,
    pub parenic_parameters: ParenicEtherealParameters,
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
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature.template_parameters(db),
        )?;
        let parenic_parameters = ParenicEtherealParameters::from_declarative(
            db,
            declarative_signature.parenic_parameters(db),
        )?;
        let return_ty = EtherealTerm::ty_from_declarative(db, declarative_signature.return_ty(db))?;
        let ty = EtherealTermRitchie::new(
            db,
            RitchieKind::FnType,
            parenic_parameters.iter().copied(),
            return_ty,
        )?
        .into();
        Ok(Self::new(
            db,
            path,
            self_ty,
            template_parameters,
            parenic_parameters,
            return_ty,
            ty,
        ))
    }
}
