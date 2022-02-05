mod atom;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

use common::*;

pub use crate::error::{AstError, AstResult, AstResultArc};
pub use expr::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstText};
pub use stmt::RawStmt;
pub use syntax_types::InitKind;
use transform::AstTransformer;

use scope::ScopePtr;
use syntax_types::*;

use word::{CustomIdentifier, Identifier, StmtKeyword};

use crate::error::{ast_err, ast_error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ast {
    TypeDef {
        ident: CustomIdentifier,
        kind: TyKind,
        generics: Vec<GenericPlaceholderKind>,
    },
    MainDef,
    DatasetConfig,
    FuncDef {
        kind: FuncKind,
        decl: FuncDecl,
    },
    PatternDef,
    Use {
        ident: CustomIdentifier,
        scope: ScopePtr,
    },
    MembDef {
        ident: CustomIdentifier,
        kind: MembKind,
    },
    Stmt(RawStmt),
}

impl From<RawStmt> for Ast {
    fn from(stmt: RawStmt) -> Self {
        Self::Stmt(stmt)
    }
}
