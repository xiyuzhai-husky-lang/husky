pub mod field;
pub mod idx;

use self::{field::FieldPlace, idx::PlaceIdx};
use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Place {
    Idx(PlaceIdx),
    Svar(EthSvar),
    Field(FieldPlace),
}

impl Place {
    pub fn show_info(self, db: &::salsa::Db, registry: &PlaceRegistry) -> String {
        use salsa::DebugWithDb;

        match self {
            Place::Idx(idx) => format!("{:?}", registry[idx].debug_with(db)),
            Place::Svar(_) => todo!(),
            Place::Field(_) => todo!(),
        }
    }
}
