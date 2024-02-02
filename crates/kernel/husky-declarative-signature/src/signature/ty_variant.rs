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
pub enum TypeVariantDecTemplate {
    Props(EnumPropsVariantDecTemplate),
    Unit(EnumUnitTypeVariantDecTemplate),
    Tuple(EnumTupleVariantDecTemplate),
}

pub(crate) fn variant_signature_template_from_decl(
    _db: &::salsa::Db,
    decl: TypeVariantSynDecl,
) -> DeclarativeSignatureResult<TypeVariantDecTemplate> {
    match decl {
        TypeVariantSynDecl::Props(_) => todo!(),
        TypeVariantSynDecl::Unit(_) => todo!(),
        TypeVariantSynDecl::Tuple(_) => todo!(),
    }
}

impl TypeVariantDecTemplate {}

impl HasDecTemplate for TypeVariantPath {
    type DecTemplate = TypeVariantDecTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DecTemplate> {
        ty_variant_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_variant_syn_declarative_signature_template(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> DeclarativeSignatureResult<TypeVariantDecTemplate> {
    Ok(
        match path.parent_ty_path(db).declarative_signature_template(db)? {
            TypeDecTemplate::Enum(parent_ty_template) => match path.syn_decl(db)? {
                TypeVariantSynDecl::Props(decl) => {
                    EnumPropsVariantDecTemplate::from_decl(db, parent_ty_template, decl)?.into()
                }
                TypeVariantSynDecl::Unit(decl) => {
                    EnumUnitTypeVariantDecTemplate::from_decl(db, parent_ty_template, decl)?.into()
                }
                TypeVariantSynDecl::Tuple(decl) => {
                    EnumTupleVariantDecTemplate::from_decl(db, parent_ty_template, decl)?.into()
                }
            },
            TypeDecTemplate::Inductive(_) => todo!(),
            _ => todo!(),
        },
    )
}
