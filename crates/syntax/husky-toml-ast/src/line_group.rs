use std::convert::Infallible;

use smallvec::SmallVec;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum TomlLineGroup {
    SectionTitle {
        title: SmallVec<[Word; 2]>,
        kind: TomlSectionKind,
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
