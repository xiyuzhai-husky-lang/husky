use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EthSignatureJar)]
pub struct EnumUnitTypeVariantEthTemplate {
    pub parent_ty_template: EnumEthTemplate,
    pub self_ty: EthTerm,
}

impl EnumUnitTypeVariantEthTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.self_ty(db)
    }

    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeVariantPath,
        tmpl: EnumUnitTypeVariantDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let TypeEthTemplate::Enum(parent_ty_template) = path.parent_ty_path(db).eth_template(db)?
        else {
            unreachable!()
        };
        let self_ty = EthTerm::ty_from_dec(db, tmpl.self_ty(db))?;
        Ok(Self::new(db, parent_ty_template, self_ty))
    }
}
