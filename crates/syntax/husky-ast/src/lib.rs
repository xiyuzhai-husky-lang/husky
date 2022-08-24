mod context;
mod error;
mod expr;
mod field;
mod query;
mod stmt;
mod transform;
mod xml;

pub use crate::error::{AstError, AstErrorVariant, AstResult, AstResultArc};
pub use context::*;
pub use expr::*;
pub use field::*;
pub use query::{AstQueryGroup, AstQueryGroupStorage, AstSalsaQueryGroup, AstText};
pub use stmt::*;
pub use transform::*;
pub use xml::*;

use error::*;
use husky_atom::*;
use husky_check_utils::*;
use husky_defn_head::*;
use husky_dev_utils::*;
use husky_entity_kind::*;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_file::FilePtr;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_print_utils::*;
use husky_text::*;
use husky_word::{CustomIdentifier, IdentDict, Identifier, Paradigm, StmtKeyword};
use std::sync::Arc;

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
        spatial_parameters: IdentDict<SpatialParameter>,
    },
    MainDefnHead,
    CallFormDefnHead {
        ident: RangedCustomIdentifier,
        paradigm: Paradigm,
        spatial_parameters: IdentDict<SpatialParameter>,
        parameters: Arc<Vec<Parameter>>,
        output_ty: RangedEntityRoute,
        output_liason: OutputLiason,
        opt_this_liason: Option<ParameterModifier>,
    },
    FeatureDefnHead {
        paradigm: Paradigm,
        ident: RangedCustomIdentifier,
        output_ty: RangedEntityRoute,
    },
    FieldDefnHead {
        liason: MemberLiason,
        ranged_ident: RangedCustomIdentifier,
        field_ty: RangedEntityRoute,
        field_ast_kind: FieldAstKind,
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
