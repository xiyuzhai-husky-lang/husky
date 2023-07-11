mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TypeVariantEtherealSignatureTemplate {
    Props(EnumPropsTypeVariantEtherealSignatureTemplate),
    Unit(EnumUnitTypeVariantEtherealSignatureTemplate),
    Tuple(EnumTupleTypeVariantEtherealSignatureTemplate),
}

impl HasEtherealSignatureTemplate for TypeVariantPath {
    type EtherealSignatureTemplate = TypeVariantEtherealSignatureTemplate;

    fn ethereal_signature_template(
        self,
        db: &dyn EtherealSignatureDb,
    ) -> EtherealSignatureResult<Self::EtherealSignatureTemplate> {
        ty_variant_ethereal_signature_template(db, self)
    }
}

#[salsa::tracked(jar = EtherealSignatureJar)]
fn ty_variant_ethereal_signature_template(
    db: &dyn EtherealSignatureDb,
    path: TypeVariantPath,
) -> EtherealSignatureResult<TypeVariantEtherealSignatureTemplate> {
    Ok(match path.declarative_signature_template(db)? {
        TypeVariantDeclarativeSignatureTemplate::Props(_) => todo!(),
        TypeVariantDeclarativeSignatureTemplate::Unit(_) => todo!(),
        TypeVariantDeclarativeSignatureTemplate::Tuple(declarative_signature_template) => {
            EnumTupleTypeVariantEtherealSignatureTemplate::from_declarative(
                db,
                path,
                declarative_signature_template,
            )?
            .into()
        }
    })
}
