use crate::*;

#[salsa::interned(db = ValDb, jar = ValJar)]
pub struct Val {
    data: ValData,
}

impl Val {
    pub unsafe fn from_raw(raw: u32) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ValData {}
