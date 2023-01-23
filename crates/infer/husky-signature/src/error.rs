use crate::*;
use outcome::Outcome;
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
pub enum SignatureAbortion {
    ExprError,
    TermError,
}

impl<Db: ?Sized + SignatureDb> salsa::DebugWithDb<Db> for SignatureAbortion {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}

pub type SignatureOutcome<T> = Outcome<T, Infallible, SignatureAbortion>;
pub type SignatureOutcomeBorrowed<'a, T> = Outcome<T, Infallible, &'a SignatureAbortion>;
