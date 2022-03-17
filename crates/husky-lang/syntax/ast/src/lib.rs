mod atom;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

use common::*;
use text::TextRange;

pub use crate::error::{AstError, AstResult, AstResultArc};
pub use expr::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::{RawBoundary, RawBranchKind, RawLoopKind, RawStmt, RawStmtKind};
use transform::AstTransformer;
use vm::InitKind;

use scope::ScopePtr;
use syntax_types::*;

use word::{CustomIdentifier, Identifier, StmtKeyword};

use crate::error::{err, error};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub kind: AstKind,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstKind {
    TypeDef {
        ident: CustomIdentifier,
        kind: TyKind,
        generics: Vec<GenericPlaceholder>,
    },
    MainDef,
    DatasetConfig,
    RoutineDef {
        routine_kind: RoutineKind,
        routine_head: RoutineHead,
    },
    PatternDef,
    Use {
        ident: CustomIdentifier,
        scope: ScopePtr,
    },
    MembDef {
        ident: CustomIdentifier,
        memb_kind: MembKind,
    },
    Stmt(RawStmt),
}

impl From<RawStmt> for Ast {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            kind: AstKind::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for AstKind {
    fn from(stmt: RawStmt) -> Self {
        AstKind::Stmt(stmt)
    }
}
