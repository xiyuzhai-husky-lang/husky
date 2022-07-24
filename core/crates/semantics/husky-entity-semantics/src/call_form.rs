use std::borrow::Cow;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CallFormSource {
    Func { stmts: Avec<FuncStmt> },
    Proc { stmts: Avec<ProcStmt> },
    Lazy { stmts: Avec<LazyStmt> },
    Static(LinkageDeprecated),
}

impl From<LinkageDeprecated> for CallFormSource {
    fn from(linkage: LinkageDeprecated) -> Self {
        CallFormSource::Static(linkage)
    }
}
