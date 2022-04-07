mod atom;
mod context;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

use std::sync::Arc;

pub use crate::error::{AstError, AstResult, AstResultArc};
pub use atom::*;
pub use context::AstContext;
pub use expr::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::{RawBoundary, RawBranchKind, RawLoopKind, RawStmt, RawStmtKind};
pub use transform::*;

use crate::error::{err, error};
use check_utils::*;
use dev_utils::*;
use entity_route::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use entity_syntax::RawTyKind;
use print_utils::*;
use text::TextRange;
use vm::{InitKind, InputContract, MembAccessContract};
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
        generic_placeholders: IdentMap<GenericPlaceholder>,
    },
    MainDefn,
    RoutineDefnHead(RoutineDefnHead),
    PatternDefnHead,
    FeatureDecl {
        ident: CustomIdentifier,
        ty: RangedEntityRoute,
    },
    MembFeatureDefnHead {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
    },
    MembRoutineDefnHead(MembRoutineDefnHead),
    Use {
        ident: CustomIdentifier,
        scope: EntityRoutePtr,
    },
    MembVarDefn(MembVarDefn),
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: CustomIdentifier,
        variant_class: EnumVariantKind,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutineDefnHead {
    pub ident: CustomIdentifier,
    pub routine_kind: RoutineKind,
    pub generic_placeholders: IdentMap<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembRoutineDefnHead {
    pub ident: CustomIdentifier,
    pub routine_kind: RoutineKind,
    pub this_contract: InputContract,
    pub generics: IdentMap<GenericPlaceholder>,
    pub input_placeholders: Arc<Vec<InputPlaceholder>>,
    pub output: RangedEntityRoute,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MembVarDefn {
    pub ident: CustomIdentifier,
    pub contract: MembAccessContract,
    pub ty: EntityRoutePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputPlaceholder {
    pub ident: CustomIdentifier,
    pub contract: InputContract,
    pub ranged_ty: RangedEntityRoute,
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
