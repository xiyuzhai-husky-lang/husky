mod atomic_decl;
pub(crate) mod atomic_stmt;
mod stack;

use atomic_stmt::AtomicStmt;

use scope::{ScopeId, ScopeKind};
pub(crate) use stack::AtomStack;
use token::Token;
use word::{CustomIdentifier, StmtKeyword};

use crate::{parser::ScopeLRParser, scope_proxy::ScopeProxy, types::*, AtomResult, *};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicLineGroup {
    TypeDef {
        ident: CustomIdentifier,
        kind: TypeKind,
        args: Vec<GenericPlaceholder>,
    },
    MainDef,
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

impl AtomicLineGroup {
    pub fn parse_stmt(
        scope_proxy: ScopeProxy,
        keyword: Option<(StmtKeyword, TextRange)>,
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

    pub fn parse_ty(scope_proxy: ScopeProxy, tokens: &[Token]) -> AtomResult<ScopeId> {
        let result = ScopeLRParser::new(scope_proxy, tokens.into()).parse()?;
        if result.len() == 0 {
            panic!()
        }
        if result.len() > 1 {
            atom_err!(
                text::group_text_range(&result[1..]),
                "unexpected atoms in memb var"
            )?
        } else {
            match result[0].kind {
                AtomKind::Scope(scope, ScopeKind::Type) => Ok(scope),
                _ => atom_err!(
                    text::group_text_range(&result),
                    "unexpected atoms in memb var"
                )?,
            }
        }
    }
}
