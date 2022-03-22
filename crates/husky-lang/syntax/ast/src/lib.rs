mod atom;
mod env;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

use common::*;
use text::TextRange;

pub use crate::error::{AstError, AstResult, AstResultArc};
use env::Env;
pub use expr::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::{RawBoundary, RawBranchKind, RawLoopKind, RawStmt, RawStmtKind};
pub use transform::*;
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
        kind: RawTyKind,
        generics: Vec<GenericPlaceholder>,
    },
    MainDecl,
    DatasetConfig,
    RoutineDecl {
        routine_kind: RoutineKind,
        routine_head: RoutineHead,
    },
    MembRoutineDecl {
        routine_kind: RoutineKind,
        memb_routine_head: MembRoutineHead,
    },
    PatternDef,
    Use {
        ident: CustomIdentifier,
        scope: ScopePtr,
    },
    MembVar {
        ident: CustomIdentifier,
        signature: MembVarSignature,
    },
    Stmt(RawStmt),
    EnumVariant {
        ident: CustomIdentifier,
        raw_variant_kind: RawEnumVariantKind,
    },
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
