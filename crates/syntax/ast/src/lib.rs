mod atom;
mod env;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

pub use crate::error::{AstError, AstResult, AstResultArc};
pub use expr::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::{RawBoundary, RawBranchKind, RawLoopKind, RawStmt, RawStmtKind};
pub use transform::*;

use crate::error::{err, error};
use check_utils::*;
use dev_utils::*;
use env::Env;
use print_utils::*;
use scope::{RangedScope, ScopePtr};
use syntax_types::*;
use text::TextRange;
use vm::InitKind;
use word::{CustomIdentifier, Identifier, StmtKeyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub kind: AstKind,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstKind {
    TypeDecl {
        ident: CustomIdentifier,
        kind: RawTyKind,
        generics: Vec<GenericPlaceholder>,
    },
    MainDecl,
    RoutineDecl {
        routine_kind: RoutineKind,
        routine_head: RoutineHead,
    },
    PatternDecl,
    FeatureDecl {
        ident: CustomIdentifier,
        ty: RangedScope,
    },
    MembFeatureDecl {
        ident: CustomIdentifier,
        ty: ScopePtr,
    },
    MembRoutineDecl {
        routine_kind: RoutineKind,
        memb_routine_head: MembRoutineHead,
    },
    Use {
        ident: CustomIdentifier,
        scope: ScopePtr,
    },
    MembVar {
        ident: CustomIdentifier,
        signature: MembAccessSignature,
    },
    DatasetConfig,
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
