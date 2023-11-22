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

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        match self {
            TypeVariantHirDefn::Unit(hir_defn) => hir_defn.dependencies(db),
            TypeVariantHirDefn::Tuple(hir_defn) => hir_defn.dependencies(db),
            TypeVariantHirDefn::Props(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        match self {
            TypeVariantHirDefn::Unit(hir_defn) => hir_defn.version_stamp(db),
            TypeVariantHirDefn::Tuple(hir_defn) => hir_defn.version_stamp(db),
            TypeVariantHirDefn::Props(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for TypeVariantPath {
    type HirDefn = TypeVariantHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(match self.hir_decl(db)? {
            TypeVariantHirDecl::Props(hir_decl) => {
                EnumPropsVariantHirDefn::new(db, self, hir_decl).into()
            }
            TypeVariantHirDecl::Unit(hir_decl) => {
                EnumUnitVariantHirDefn::new(db, self, hir_decl).into()
            }
            TypeVariantHirDecl::Tuple(hir_decl) => {
                EnumTupleVariantHirDefn::new(db, self, hir_decl).into()
            }
        })
    }
}
