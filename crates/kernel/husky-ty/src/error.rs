use crate::*;
use thiserror::Error;

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeError {}

impl<Db: TypeDb + ?Sized> salsa::DebugWithDb<Db> for TypeError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
