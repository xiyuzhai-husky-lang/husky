mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TypeVariantEthTemplate {
    Props(EnumPropsVariantEthTemplate),
    Unit(EnumUnitTypeVariantEthTemplate),
    Tuple(EnumTupleVariantEthTemplate),
}

impl TypeVariantEthTemplate {
    pub fn self_ty(self, _db: &::salsa::Db) -> EtherealTerm {
        match self {
            TypeVariantEthTemplate::Props(_) => todo!(),
            TypeVariantEthTemplate::Unit(_) => todo!(),
            TypeVariantEthTemplate::Tuple(_) => todo!(),
        }
    }

    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        match self {
            TypeVariantEthTemplate::Props(slf) => slf.instance_constructor_ty(db),
            TypeVariantEthTemplate::Unit(slf) => slf.instance_constructor_ty(db),
            TypeVariantEthTemplate::Tuple(slf) => slf.instance_constructor_ty(db),
        }
    }
}

impl HasEthTemplate for TypeVariantPath {
    type EthTemplate = TypeVariantEthTemplate;

    fn ethereal_signature_template(
        self,
        db: &::salsa::Db,
    ) -> EtherealSignatureResult<Self::EthTemplate> {
        ty_variant_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_variant_ethereal_signature_template(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> EtherealSignatureResult<TypeVariantEthTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeVariantDecTemplate::Props(_) => todo!(),
        TypeVariantDecTemplate::Unit(declarative_signature_template) => {
            EnumUnitTypeVariantEthTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
        TypeVariantDecTemplate::Tuple(declarative_signature_template) => {
            EnumTupleVariantEthTemplate::from_declarative(db, path, declarative_signature_template)?
                .into()
        }
    })
}
