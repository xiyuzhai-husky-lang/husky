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
pub enum VariantDefn {
    Unit(UnitVariantDefn),
    Tuple(TupleVariantDefn),
    Props(PropsVariantDefn),
}

impl VariantDefn {
    pub fn decl(self, _db: &dyn DefnDb) -> Decl {
        todo!()
    }

    pub fn path(self, _db: &dyn DefnDb) -> TypeVariantPath {
        todo!()
    }
}
