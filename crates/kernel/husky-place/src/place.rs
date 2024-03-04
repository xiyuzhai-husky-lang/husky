pub mod field;
pub mod idx;

use self::{field::FieldPlace, idx::PlaceIdx};
use crate::*;
use husky_eth_term::term::svar::EthSvar;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EthPlace {
    Idx(PlaceIdx),
    Svar(EthSvar),
    Field(FieldPlace),
}

impl EthPlace {
    pub fn show_info(self, db: &::salsa::Db, registry: &PlaceRegistry) -> String {
        use salsa::DebugWithDb;

        match self {
            EthPlace::Idx(idx) => format!("{:?}", registry[idx].debug_with(db)),
            EthPlace::Svar(_) => todo!(),
            EthPlace::Field(_) => todo!(),
        }
    }
}
