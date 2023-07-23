mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantHirDefn {
    Unit(EnumUnitVariantHirDefn),
    Tuple(EnumTupleVariantHirDefn),
    Props(EnumPropsVariantHirDefn),
}

impl TypeVariantHirDefn {
    pub fn hir_decl(self, _db: &dyn HirDefnDb) -> HirDecl {
        todo!()
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> TypeVariantPath {
        todo!()
    }
}
