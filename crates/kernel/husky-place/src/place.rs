pub mod field;
pub mod idx;

use self::field::FieldPlace;
use crate::*;

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Place {
    Idx(PlaceIdx),
    Svar(EthSvar),
    Field(FieldPlace),
}
