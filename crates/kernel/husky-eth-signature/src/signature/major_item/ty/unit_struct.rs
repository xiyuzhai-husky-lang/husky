use super::*;
use husky_dec_signature::UnitStructTypeDecTemplate;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct UnitStructTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_ty: EthTerm,
}

impl UnitStructTypeEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: UnitStructTypeDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_declarative(db, tmpl.template_parameters(db))?;
        let self_ty = EthTerm::ty_from_declarative(db, tmpl.self_ty(db))?;
        Ok(Self::new(db, path, template_parameters, self_ty))
    }
}

impl UnitStructTypeEthTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.self_ty(db)
    }
}
