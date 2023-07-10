use std::convert::Infallible;

use smallvec::SmallVec;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub enum TomlLineGroup {
    SectionTitle {
        title: SmallVec<[Coword; 2]>,
        kind: TomlSectionKind,
    },
    KeyValue(Coword, Option<TomlExprIdx>),
    Comment,
    Err,
}

impl std::ops::FromResidual<Result<Infallible, TomlAstError>> for TomlLineGroup {
    fn from_residual(residual: Result<Infallible, TomlAstError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(_e) => TomlLineGroup::Err,
        }
    }
}
