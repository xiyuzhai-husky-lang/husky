mod atom;
mod env;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

pub use crate::error::{AstError, AstResult, AstResultArc};
pub use atom::*;
use entity_syntax::RawTyKind;
pub use expr::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::{RawBoundary, RawBranchKind, RawLoopKind, RawStmt, RawStmtKind};
pub use transform::*;

use crate::error::{err, error};
use check_utils::*;
use dev_utils::*;
use entity_route::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use env::Env;
use print_utils::*;
use syntax_types::*;
use text::TextRange;
use vm::InitKind;
use word::{CustomIdentifier, IdentMap, Identifier, StmtKeyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub kind: AstKind,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstKind {
    TypeDefnHead {
        ident: CustomIdentifier,
        kind: RawTyKind,
        generic_placeholders: IdentMap<GenericPlaceholderKind>,
    },
    MainDefn,
    RoutineDefnHead {
        routine_class: RoutineClass,
        routine_head: RoutineHead,
    },
    PatternDefnHead,
    FeatureDecl {
        ident: CustomIdentifier,
        ty: RangedEntityRoute,
    },
    MembFeatureDefnHead {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
    },
    MembRoutineDefnHead {
        routine_kind: RoutineClass,
        memb_routine_head: MembRoutineHead,
    },
    Use {
        ident: CustomIdentifier,
        scope: EntityRoutePtr,
    },
    MembVarDefn {
        ident: CustomIdentifier,
        signature: MembAccessDecl,
    },
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: CustomIdentifier,
        variant_class: EnumVariantClass,
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
