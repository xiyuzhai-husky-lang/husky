mod props;
mod tuple;
mod unit;

pub use self::props::*;
pub use self::tuple::*;
pub use self::unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantNodeDefn {
    Unit(UnitVariantNodeDefn),
    Tuple(TupleVariantNodeDefn),
    Props(PropsVariantNodeDefn),
}

impl HasNodeDefn for TypeVariantNodePath {
    type NodeDefn = TypeVariantNodeDefn;

    fn node_defn(self, db: &dyn DefnDb) -> Self::NodeDefn {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DefnDb)]
#[enum_class::from_variants]
pub enum TypeVariantDefn {
    Unit(UnitVariantDefn),
    Tuple(TupleVariantDefn),
    Props(PropsVariantDefn),
}

impl TypeVariantDefn {
    pub fn decl(self, _db: &dyn DefnDb) -> Decl {
        todo!()
    }

    pub fn path(self, _db: &dyn DefnDb) -> TypeVariantPath {
        todo!()
    }
}
