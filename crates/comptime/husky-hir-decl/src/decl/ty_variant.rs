mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantHirDecl {
    Props(EnumPropsTypeVariantHirDecl),
    Unit(EnumUnitTypeVariantHirDecl),
    Tuple(EnumTupleTypeVariantHirDecl),
}

impl HasHirDecl for TypeVariantPath {
    type HirDecl = TypeVariantHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        ty_variant_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn ty_variant_hir_decl(db: &dyn HirDeclDb, path: TypeVariantPath) -> TypeVariantHirDecl {
    todo!()
    // Ok(match path.declarative_signature_template(db)? {
    //     TypeVariantDeclarativeSignatureTemplate::Props(_) => todo!(),
    //     TypeVariantDeclarativeSignatureTemplate::Unit(declarative_signature_template) => {
    //         EnumUnitTypeVariantHirDecl::from_declarative(db, path, declarative_signature_template)?
    //             .into()
    //     }
    //     TypeVariantDeclarativeSignatureTemplate::Tuple(declarative_signature_template) => {
    //         EnumTupleTypeVariantHirDecl::from_declarative(db, path, declarative_signature_template)?
    //             .into()
    //     }
    // })
}
