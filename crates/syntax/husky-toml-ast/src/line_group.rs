use std::convert::Infallible;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TomlLineGroup {
    SectionTitle {
        title: Vec<Word>,
        is_scattered: bool,
    },
    KeyValue(Word, Option<TomlExprIdx>),
    Comment,
    Err,
}

impl const std::ops::FromResidual<Result<Infallible, TomlAstError>> for TomlLineGroup {
    fn from_residual(residual: Result<Infallible, TomlAstError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => TomlLineGroup::Err,
        }
    }
}
