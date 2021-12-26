mod atomic_decl;
pub(crate) mod atomic_stmt;
mod stack;
use atomic_decl::AtomicDecl;
use atomic_stmt::AtomicStmt;

pub(crate) use stack::AtomStack;
use token::Token;
use word::StmtKeyword;

use crate::{parser::ScopeLRParser, scope_proxy::ScopeProxy, AtomResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicLineGroup {
    Stmt(AtomicStmt),
    Decl(AtomicDecl),
}

impl From<AtomicStmt> for AtomicLineGroup {
    fn from(_: AtomicStmt) -> Self {
        todo!()
    }
}

impl AtomicLineGroup {
    pub fn stmt(
        scope_proxy: ScopeProxy,
        keyword: Option<StmtKeyword>,
        is_head: bool,
        tokens: &[Token],
    ) -> AtomResult<AtomicLineGroup> {
        Ok(AtomicStmt::new(
            keyword,
            is_head,
            ScopeLRParser::new(scope_proxy, tokens.into()).parse()?,
        )
        .into())
    }
}
