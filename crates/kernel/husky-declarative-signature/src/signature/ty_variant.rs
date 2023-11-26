mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum TypeVariantDeclarativeSignatureTemplate {
    Props(EnumPropsVariantDeclarativeSignatureTemplate),
    Unit(EnumUnitTypeVariantDeclarativeSignatureTemplate),
    Tuple(EnumTupleVariantDeclarativeSignatureTemplate),
}

pub(crate) fn variant_signature_template_from_decl(
    _db: &::salsa::Db,
    decl: TypeVariantSynDecl,
) -> DeclarativeSignatureResult<TypeVariantDeclarativeSignatureTemplate> {
    match decl {
        TypeVariantSynDecl::Props(_) => todo!(),
        TypeVariantSynDecl::Unit(_) => todo!(),
        TypeVariantSynDecl::Tuple(_) => todo!(),
        // TypeDecl::Enum(decl) => enum_declarative_signature_template(db, decl).into(),
    }
}

impl TypeVariantDeclarativeSignatureTemplate {}

impl HasDeclarativeSignatureTemplate for TypeVariantPath {
    type DeclarativeSignatureTemplate = TypeVariantDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_variant_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_variant_syn_declarative_signature_template(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> DeclarativeSignatureResult<TypeVariantDeclarativeSignatureTemplate> {
    Ok(
        match path.parent_ty_path(db).declarative_signature_template(db)? {
            TypeDeclarativeSignatureTemplate::Enum(parent_ty_template) => {
                match path.syn_decl(db)? {
                    TypeVariantSynDecl::Props(_) => todo!(),
                    TypeVariantSynDecl::Unit(decl) => {
                        EnumUnitTypeVariantDeclarativeSignatureTemplate::from_decl(
                            db,
                            parent_ty_template,
                            decl,
                        )?
                        .into()
                    }
                    TypeVariantSynDecl::Tuple(decl) => {
                        EnumTupleVariantDeclarativeSignatureTemplate::from_decl(
                            db,
                            parent_ty_template,
                            decl,
                        )?
                        .into()
                    }
                }
            }
            TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
            _ => todo!(),
        },
    )
}
