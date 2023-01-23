use crate::*;

use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureError {
    ExprError,
    TermError,
    ParameterTypeTermError(u8),
    FieldTypeTermError(u8),
    OutputTypeTermError,
}

impl<Db: ?Sized + SignatureDb> salsa::DebugWithDb<Db> for SignatureError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(&self, f)
    }
}

pub type SignatureResult<T> = Result<T, SignatureError>;
pub type SignatureResultBorrowed<'a, T> = Result<T, &'a SignatureError>;
