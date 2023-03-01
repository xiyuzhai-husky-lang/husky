use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum RuntimeExprError {
    Hellow,
}

pub type RuntimeExprResult<T> = Result<T, RuntimeExprError>;
