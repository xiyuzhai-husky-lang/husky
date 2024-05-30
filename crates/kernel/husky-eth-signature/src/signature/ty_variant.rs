pub mod enum_props_ty_variant;
pub mod enum_tuple_ty_variant;
pub mod enum_unit_ty_variant;

use self::enum_props_ty_variant::*;
use self::enum_tuple_ty_variant::*;
use self::enum_unit_ty_variant::*;
use super::*;
use husky_dec_signature::signature::ty_variant::TypeVariantDecTemplate;
use husky_entity_path::path::ty_variant::TypeVariantPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeVariantEthTemplate {
    Props(EnumPropsVariantEthTemplate),
    Unit(EnumUnitTypeVariantEthTemplate),
    Tuple(EnumTupleVariantEthTemplate),
}

impl TypeVariantEthTemplate {
    pub fn self_ty(self, _db: &::salsa::Db) -> EthTerm {
        match self {
            TypeVariantEthTemplate::Props(_) => todo!(),
            TypeVariantEthTemplate::Unit(_) => todo!(),
            TypeVariantEthTemplate::Tuple(_) => todo!(),
        }
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        match self {
            TypeVariantEthTemplate::Props(slf) => slf.instance_constructor_ty(db),
            TypeVariantEthTemplate::Unit(slf) => slf.instance_constructor_ty(db),
            TypeVariantEthTemplate::Tuple(slf) => slf.instance_constructor_ty(db),
        }
    }
}

impl HasEthTemplate for TypeVariantPath {
    type EthTemplate = TypeVariantEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        ty_variant_eth_template(db, self)
    }
}

#[salsa::tracked]
fn ty_variant_eth_template(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> EthSignatureResult<TypeVariantEthTemplate> {
    Ok(match path.dec_template(db)? {
        TypeVariantDecTemplate::EnumProps(_) => todo!(),
        TypeVariantDecTemplate::EnumUnit(dec_template) => {
            EnumUnitTypeVariantEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TypeVariantDecTemplate::EnumTuple(dec_template) => {
            EnumTupleVariantEthTemplate::from_dec(db, path, dec_template)?.into()
        }
    })
}
