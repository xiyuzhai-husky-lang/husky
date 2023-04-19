mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum VariantDeclarativeSignatureTemplate {
    Props(PropsVariantDeclarativeSignatureTemplate),
    Unit(UnitVariantDeclarativeSignatureTemplate),
    Tuple(TupleVariantDeclarativeSignatureTemplate),
}

pub(crate) fn variant_signature_template_from_decl(
    _db: &dyn DeclarativeSignatureDb,
    decl: TypeVariantDecl,
) -> DeclarativeSignatureResult<VariantDeclarativeSignatureTemplate> {
    match decl {
        TypeVariantDecl::Props(_) => todo!(),
        TypeVariantDecl::Unit(_) => todo!(),
        TypeVariantDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_ty_declarative_signature_template(db, decl).into(),
    }
}

impl VariantDeclarativeSignatureTemplate {}
