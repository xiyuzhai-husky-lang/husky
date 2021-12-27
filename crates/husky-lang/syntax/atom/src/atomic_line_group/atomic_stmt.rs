use word::StmtKeyword;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicStmt {
    pub attr: StmtAttr,
    pub atoms: Vec<Atom>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StmtAttr {
    pub keyword: Option<(StmtKeyword, TextRange)>,
    pub is_head: bool,
}

// new
impl AtomicStmt {
    pub(crate) fn new(
        keyword: Option<(StmtKeyword, TextRange)>,
        is_head: bool,
        atoms: Vec<Atom>,
    ) -> Self {
        Self {
            attr: StmtAttr { keyword, is_head },
            atoms,
        }
    }
}
