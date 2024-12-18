use super::*;
use crate::signature::major_item::ty::{r#enum::EnumEthTemplate, TypeEthTemplate};
use husky_dec_signature::signature::ty_variant::enum_tuple_ty_variant::EnumTupleVariantDecTemplate;
use husky_eth_term::term::ritchie::EthRitchie;

#[salsa::interned]
pub struct EnumTupleVariantEthTemplate {
    pub path: TypeVariantPath,
    pub parent_ty_template: EnumEthTemplate,
    pub instance_constructor_ritchie_ty: EthRitchie,
}

impl EnumTupleVariantEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeVariantPath,
        tmpl: EnumTupleVariantDecTemplate,
    ) -> EthSignatureResult<Self> {
        let TypeEthTemplate::Enum(parent_ty_template) = path.parent_ty_path(db).eth_template(db)?
        else {
            unreachable!()
        };
        let instance_constructor_ty = EthRitchie::from_dec(db, tmpl.instance_constructor_ty(db))?;
        Ok(Self::new(
            db,
            path,
            parent_ty_template,
            instance_constructor_ty,
        ))
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}
