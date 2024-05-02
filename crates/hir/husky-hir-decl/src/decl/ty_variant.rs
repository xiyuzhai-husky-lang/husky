mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;
use husky_entity_kind::TypeKind;
use husky_entity_path::path::ty_variant::TypeVariantPath;
use husky_syn_decl::decl::TypeVariantSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TypeVariantHirDecl {
    Props(EnumPropsVariantHirDecl),
    Unit(EnumUnitTypeVariantHirDecl),
    Tuple(EnumTupleVariantHirDecl),
}

impl TypeVariantHirDecl {
    pub fn path(self, _db: &::salsa::Db) -> TypeVariantPath {
        match self {
            TypeVariantHirDecl::Props(_) => todo!(),
            TypeVariantHirDecl::Unit(_) => todo!(),
            TypeVariantHirDecl::Tuple(_) => todo!(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            TypeVariantHirDecl::Props(slf) => slf.hir_eager_expr_region(db).into(),
            TypeVariantHirDecl::Unit(slf) => slf.hir_eager_expr_region(db).into(),
            TypeVariantHirDecl::Tuple(slf) => slf.hir_eager_expr_region(db).into(),
        }
    }
}

impl HasHirDecl for TypeVariantPath {
    type HirDecl = TypeVariantHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        ty_variant_hir_decl(db, self)
    }
}

#[salsa::tracked]
fn ty_variant_hir_decl(db: &::salsa::Db, path: TypeVariantPath) -> Option<TypeVariantHirDecl> {
    match path.parent_ty_path(db).ty_kind(db) {
        TypeKind::Enum => (),
        TypeKind::Inductive => return None,
        _ => unreachable!(),
    }
    Some(match path.syn_decl(db).expect("no errors for hir stage") {
        TypeVariantSynDecl::Props(syn_decl) => {
            EnumPropsVariantHirDecl::from_syn(path, syn_decl, db).into()
        }
        TypeVariantSynDecl::Tuple(syn_decl) => {
            EnumTupleVariantHirDecl::from_syn(path, syn_decl, db).into()
        }
        TypeVariantSynDecl::Unit(syn_decl) => {
            EnumUnitTypeVariantHirDecl::from_syn(path, syn_decl, db).into()
        }
    })
}
