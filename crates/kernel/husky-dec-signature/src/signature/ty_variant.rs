pub mod enum_props_ty_variant;
pub mod enum_tuple_ty_variant;
pub mod enum_unit_ty_variant;

use self::enum_props_ty_variant::*;
use self::enum_tuple_ty_variant::*;
use self::enum_unit_ty_variant::*;
use super::*;
use crate::signature::major_item::ty::r#enum::EnumDecTemplate;
use crate::signature::major_item::ty::TypeDecTemplate;
use husky_entity_path::path::ty_variant::TypeVariantPath;
use husky_syn_decl::decl::ty_variant::TypeVariantSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeVariantDecTemplate {
    EnumProps(EnumPropsVariantDecTemplate),
    EnumUnit(EnumUnitTypeVariantDecTemplate),
    EnumTuple(EnumTupleVariantDecTemplate),
}

impl TypeVariantDecTemplate {}

impl HasDecTemplate for TypeVariantPath {
    type DecTemplate = TypeVariantDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        ty_variant_dec_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn ty_variant_dec_template(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> DecSignatureResult<TypeVariantDecTemplate> {
    Ok(match path.parent_ty_path(db).dec_template(db)? {
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
    })
}
