use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Reserved(Reserved),
    UserDefined(u32),
}

impl From<u32> for Identifier {
    fn from(raw: u32) -> Self {
        Self::UserDefined(raw)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Reserved {
    I32,
    F32,
    Vec,
    Tuple,
    Debug,
    Std,
    Core,
    Fp,
    Fn,
    FnMut,
    FnOnce,
}

pub fn default_func_type() -> Reserved {
    Reserved::Fp
}
