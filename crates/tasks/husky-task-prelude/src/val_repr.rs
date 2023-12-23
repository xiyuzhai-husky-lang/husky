use shifted_unsigned_int::ShiftedU32;
use smallvec::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValReprInterface(ShiftedU32);

pub enum ValArgumentReprInterface {
    Ordinary(ValReprInterface),
    Keyed(Option<ValReprInterface>),
    Variadic(SmallVec<[ValReprInterface; 4]>),
    Branch {
        condition: Option<ValReprInterface>,
        stmts: SmallVec<[ValReprInterface; 4]>,
    },
}
