use super::*;
use husky_declarative_signature::StructureTypeDecTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct StructureTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
}

impl StructureTypeEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        declarative_signature_template: StructureTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters = EtherealTemplateParameters::from_declarative(
            db,
            declarative_signature_template.template_parameters(db),
        )?;
        // let fields = declarative_signature_template
        //     .fields(db)
        //     .iter()
        //     .copied()
        //     .map(|declarative_signature_template| {
        //         PropsFieldEthTemplate::from_declarative(
        //             db,
        //             declarative_signature_template,
        //         )
        //     })
        //     .collect::<EtherealSignatureResult<_>>()?;
        Ok(Self::new(db, path, template_parameters /* fields */))
    }
}
