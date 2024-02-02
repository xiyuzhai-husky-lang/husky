use super::*;
use husky_dec_signature::UnionTypeDecTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct UnionTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl UnionTypeEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        dec_template: UnionTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_declarative(db, dec_template.template_parameters(db))?;
        // let fields = dec_template
        //     .fields(db)
        //     .iter()
        //     .copied()
        //     .map(|dec_template| {
        //         PropsFieldEthTemplate::from_declarative(
        //             db,
        //             dec_template,
        //         )
        //     })
        //     .collect::<EtherealSignatureResult<_>>()?;
        Ok(Self::new(db, path, template_parameters))
    }
}
