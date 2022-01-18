use crate::LazyStmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub(crate) stmts: Vec<LazyStmt>,
}
