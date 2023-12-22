use super::*;
use husky_declarative_signature::UnitStructTypeDeclarativeSignatureTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct UnitStructTypeEtherealSignatureTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub self_ty: EtherealTerm,
}

impl UnitStructTypeEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        declarative_signature_template: UnitStructTypeDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        let self_ty = todo!();
        // let fields = declarative_signature_template
        //     .fields(db)
        //     .iter()
        //     .copied()
        //     .map(|declarative_signature_template| {
        //         PropsFieldEtherealSignatureTemplate::from_declarative(
        //             db,
        //             declarative_signature_template,
        //         )
        //     })
        //     .collect::<EtherealSignatureResult<_>>()?;
        Ok(Self::new(db, path, template_parameters, self_ty))
    }
}

impl UnitStructTypeEtherealSignatureTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.self_ty(db)
    }
}
