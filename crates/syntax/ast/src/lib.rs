mod context;
mod error;
mod expr;
mod field;
mod query;
mod stmt;
mod transform;
mod xml;

use std::sync::Arc;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::*;
pub use expr::*;
pub use field::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::*;
pub use transform::*;
pub use xml::*;

use atom::*;
use check_utils::*;
use defn_head::*;
use dev_utils::*;
use entity_kind::*;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use error::*;
use file::FilePtr;
use print_utils::*;
use text::*;
use vm::{InitKind, MemberLiason};
use word::{CustomIdentifier, IdentDict, Identifier, StmtKeyword};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub variant: AstVariant,
    pub range: TextRange,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstVariant {
    TypeDefnHead {
        ident: RangedCustomIdentifier,
        kind: TyKind,
        generic_parameters: IdentDict<SpatialParameter>,
    },
    MainDefn,
    CallFormDefnHead(CallableDefnHead),
    PatternDefnHead,
    FeatureDecl {
        ident: RangedCustomIdentifier,
        ty: RangedEntityRoute,
    },
    FieldDefnHead {
        liason: MemberLiason,
        ranged_ident: RangedCustomIdentifier,
        ty: RangedEntityRoute,
        field_kind: FieldKind,
    },
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
    Visual,
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
            variant: AstVariant::Stmt(stmt),
        }
    }
}

impl From<RawStmt> for AstVariant {
    fn from(stmt: RawStmt) -> Self {
        AstVariant::Stmt(stmt)
    }
}
