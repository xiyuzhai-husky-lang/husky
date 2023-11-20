mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantHirDefn {
    Unit(EnumUnitVariantHirDefn),
    Tuple(EnumTupleVariantHirDefn),
    Props(EnumPropsVariantHirDefn),
}

impl TypeVariantHirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> TypeVariantPath {
        match self {
            TypeVariantHirDefn::Unit(hir_defn) => hir_defn.path(db),
            TypeVariantHirDefn::Tuple(hir_defn) => hir_defn.path(db),
            TypeVariantHirDefn::Props(hir_defn) => hir_defn.path(db),
        }
    }

    pub fn hir_decl(self, _db: &dyn HirDefnDb) -> HirDecl {
        todo!()
    }
}

// impl HasHirDefn for TypeVariantPath {
//     type HirDefn = TypeVariantHirDefn;

//     fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
//         todo!()
//     }
// }
