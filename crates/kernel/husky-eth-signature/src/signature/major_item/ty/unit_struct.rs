use super::*;
use husky_dec_signature::signature::major_item::ty::unit_struct::UnitStructDecTemplate;

#[salsa::interned]
pub struct UnitStructEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_ty: EthTerm,
}

impl UnitStructEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypePath,
        tmpl: UnitStructDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, tmpl.template_parameters(db))?;
        let self_ty = EthTerm::ty_from_dec(db, tmpl.self_ty(db))?;
        Ok(Self::new(db, path, template_parameters, self_ty))
    }
}

impl UnitStructEthTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.self_ty(db)
    }
}
