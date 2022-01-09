mod atomic_decl;
pub(crate) mod atomic_stmt;
mod stack;

use atomic_stmt::AtomicStmt;

use hir::*;
use scope::{ScopeId, ScopeKind};
pub(crate) use stack::AtomStack;
use token::{Special, Token};
use word::{CustomIdentifier, StmtKeyword};

use super::{parser::ScopeLRParser, symbol_proxy::SymbolProxy, *};
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicLineGroup {
    TypeDef {
        ident: CustomIdentifier,
        kind: TypeKind,
        args: Vec<SpaceParamKind>,
    },
    MainDef,
    DatasetConfig,
    FuncDef {
        kind: FuncKind,
        decl: FuncDecl,
    },
    Use {
        ident: CustomIdentifier,
        scope: ScopeId,
    },
    MembDef {
        ident: CustomIdentifier,
        kind: MembKind,
    },
    Stmt(AtomicStmt),
}

impl From<AtomicStmt> for AtomicLineGroup {
    fn from(stmt: AtomicStmt) -> Self {
        Self::Stmt(stmt)
    }
}

pub fn parse_stmt(
    env: hir::Env,
    scope_proxy: SymbolProxy,
    keyword: Option<(StmtKeyword, TextRange)>,
    tokens: &[Token],
) -> AstResult<Ast> {
    let is_head = tokens.last().unwrap().kind == TokenKind::Special(Special::Colon);
    todo!()
    // Ok(AtomicStmt::parse(
    //     env,
    //     scope_proxy,
    //     keyword,
    //     is_head,
    //     if is_head {
    //         &tokens[0..(tokens.len() - 1)]
    //     } else {
    //         tokens
    //     },
    // )?
    // .into())
}
