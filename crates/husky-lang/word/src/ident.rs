use crate::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Identifier {
    Builtin(BuiltinIdentifier),
    UserDefined(UserDefinedIdentifier),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserDefinedIdentifier(u32);

impl From<UserDefinedIdentifier> for Identifier {
    fn from(ident: UserDefinedIdentifier) -> Self {
        Self::UserDefined(ident)
    }
}

impl From<u32> for UserDefinedIdentifier {
    fn from(raw: u32) -> Self {
        UserDefinedIdentifier(raw)
    }
}

impl From<u32> for Identifier {
    fn from(raw: u32) -> Self {
        UserDefinedIdentifier(raw).into()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BuiltinIdentifier {
    I32,
    F32,
    Vec,
    Tuple,
    Debug,
    Std,
    Core,
    Rp,
    Rt,
    RtMut,
    RtOnce,
}

pub fn default_routine_type() -> BuiltinIdentifier {
    BuiltinIdentifier::Rp
}
