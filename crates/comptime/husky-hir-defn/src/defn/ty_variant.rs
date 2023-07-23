mod props;
mod tuple;
mod unit;

pub use self::props::*;
pub use self::tuple::*;
pub use self::unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantHirNodeDefn {
    Unit(UnitVariantHirNodeDefn),
    Tuple(TupleVariantHirNodeDefn),
    Props(PropsVariantHirNodeDefn),
}

impl TypeVariantHirNodeDefn {
    pub fn syn_node_decl(self, db: &dyn HirDefnDb) -> TypeVariantNodeDecl {
        todo!()
    }
}

impl HasHirNodeDefn for TypeVariantHirNodePath {
    type NodeDefn = TypeVariantHirNodeDefn;

    fn syn_node_defn(self, db: &dyn HirDefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantHirDefn {
    Unit(UnitVariantHirDefn),
    Tuple(TupleVariantHirDefn),
    Props(PropsVariantHirDefn),
}

impl TypeVariantHirDefn {
    pub fn decl(self, _db: &dyn HirDefnDb) -> Decl {
        todo!()
    }

    pub fn path(self, _db: &dyn HirDefnDb) -> TypeVariantPath {
        todo!()
    }
}
