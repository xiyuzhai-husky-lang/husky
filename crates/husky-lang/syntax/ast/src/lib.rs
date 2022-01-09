mod atom;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

use common::*;

pub use crate::error::{AstError, AstResult, AstResultArc};
pub use expr::*;
pub use hir::InitKind;
pub use query::{AstQuery, AstQueryStorage};
pub use stmt::Stmt;
use transform::AstTransformer;

use hir::*;
use scope::ScopeId;

use atom::BinaryOpr;
use word::{CustomIdentifier, Identifier, StmtKeyword};

use crate::error::{ast_err, ast_error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ast {
    TypeDef {
        ident: CustomIdentifier,
        kind: TypeKind,
        generics: Vec<SpaceParamKind>,
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
        scope: ScopeId,
    },
    MembDef {
        ident: CustomIdentifier,
        kind: MembKind,
    },
    Stmt(Stmt),
}

impl From<Stmt> for Ast {
    fn from(stmt: Stmt) -> Self {
        Self::Stmt(stmt)
    }
}
