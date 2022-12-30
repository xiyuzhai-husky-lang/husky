mod props;
mod tuple;
mod unit;

pub use props::*;
pub use tuple::*;
pub use unit::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VariantDefn {
    Unit(UnitVariantDefn),
    Tuple(TupleVariantDefn),
    Props(PropsVariantDefn),
}

impl VariantDefn {
    pub fn decl(self, db: &dyn DefnDb) -> Decl {
        todo!()
    }

    pub fn path(self, db: &dyn DefnDb) -> VariantPath {
        todo!()
    }
}
