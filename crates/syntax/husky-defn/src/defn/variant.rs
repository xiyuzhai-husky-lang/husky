mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

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
