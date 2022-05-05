use super::*;

impl From<SuffixOpr> for Opr {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                               // ++
    Decr,                               // --
    MayReturn,                          // ?
    MembAccess(RangedCustomIdentifier), // .
    WithType(EntityRoutePtr),           // :
}

impl SuffixOpr {
    pub fn act_on_primitive(&self, opd: PrimitiveValue) -> PrimitiveValue {
        todo!()
    }
}
