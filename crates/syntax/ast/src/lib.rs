mod context;
mod error;
mod expr;
mod query;
mod stmt;
mod transform;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::AstContext;
pub use expr::*;
use file::FilePtr;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::*;
pub use transform::*;

use atom::*;
use check_utils::*;
use defn_head::*;
use dev_utils::*;
use entity_kind::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use error::*;
use print_utils::*;
use text::*;
use vm::InitKind;
use word::{CustomIdentifier, IdentDict, Identifier, StmtKeyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub variant: AstKind,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstKind {
    TypeDefnHead {
        ident: RangedCustomIdentifier,
        kind: TyKind,
        generic_placeholders: IdentDict<GenericPlaceholder>,
    },
    MainDefn,
    RoutineDefnHead(RoutineDefnHead),
    PatternDefnHead,
    FeatureDecl {
        ident: RangedCustomIdentifier,
        ty: RangedEntityRoute,
    },
    TypeMethodDefnHead(TypeMethodDefnHead),
    TypeAssociatedRoutineDefnHead(RoutineDefnHead),
    FieldDefnHead(FieldDefnHead),
    DatasetConfigDefnHead,
    Stmt(RawStmt),
    EnumVariantDefnHead {
        ident: RangedCustomIdentifier,
        variant_class: EnumVariantKind,
    },
    Use {
        use_variant: UseVariant,
    },
    Submodule {
        ident: RangedCustomIdentifier,
        source_file: FilePtr,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UseVariant {
    Route { route: EntityRoutePtr },
    All { parent: EntityRoutePtr },
}

impl From<RawStmt> for Ast {
    fn from(stmt: RawStmt) -> Self {
        Self {
            range: stmt.range,
            variant: AstKind::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for AstKind {
    fn from(stmt: RawStmt) -> Self {
        AstKind::Stmt(stmt)
    }
}
