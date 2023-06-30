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
pub enum TypeVariantDeclarativeSignatureTemplate {
    Props(EnumPropsTypeVariantDeclarativeSignatureTemplate),
    Unit(EnumUnitVariantDeclarativeSignatureTemplate),
    Tuple(EnumTupleTypeVariantDeclarativeSignatureTemplate),
}

pub(crate) fn variant_signature_template_from_decl(
    _db: &dyn DeclarativeSignatureDb,
    decl: TypeVariantDecl,
) -> DeclarativeSignatureResult<TypeVariantDeclarativeSignatureTemplate> {
    match decl {
        TypeVariantDecl::Props(_) => todo!(),
        TypeVariantDecl::Unit(_) => todo!(),
        TypeVariantDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_declarative_signature_template(db, decl).into(),
    }
}

impl TypeVariantDeclarativeSignatureTemplate {}

impl HasDeclarativeSignatureTemplate for TypeVariantPath {
    type DeclarativeSignatureTemplate = TypeVariantDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_variant_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_variant_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    path: TypeVariantPath,
) -> DeclarativeSignatureResult<TypeVariantDeclarativeSignatureTemplate> {
    Ok(
        match path.parent_ty_path(db).declarative_signature_template(db)? {
            TypeDeclarativeSignatureTemplate::Enum(parent_ty_template) => match path.decl(db)? {
                TypeVariantDecl::Props(_) => todo!(),
                TypeVariantDecl::Unit(_) => todo!(),
                TypeVariantDecl::Tuple(decl) => {
                    EnumTupleTypeVariantDeclarativeSignatureTemplate::from_decl(
                        db,
                        parent_ty_template,
                        decl,
                    )?
                    .into()
                }
            },
            TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
            _ => todo!(),
        },
    )
}
