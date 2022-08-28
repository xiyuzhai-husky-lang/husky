use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CallFormSource {
    Func { stmts: Avec<FuncStmt> },
    Proc { stmts: Avec<ProcStmt> },
    Lazy { stmts: Avec<LazyStmt> },
    Static(__Linkage),
}

impl From<__Linkage> for CallFormSource {
    fn from(linkage: __Linkage) -> Self {
        CallFormSource::Static(linkage)
    }
}
