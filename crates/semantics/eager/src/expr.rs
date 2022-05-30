mod opn;
mod parser;

use crate::*;
use ast::{AstIter, RawExprArena, RawExprIdx};
use file::FilePtr;
use infer_qualifier::{EagerQualifiedTy, EagerQualifier};
use infer_total::InferQueryGroup;
pub use opn::*;
pub(crate) use parser::EagerExprParser;
use std::sync::Arc;

use entity_route::EntityRoutePtr;
use semantics_error::SemanticResultArc;
use text::{RangedCustomIdentifier, TextRange};
use vm::{Binding, CopyableValue, EagerContract, InstructionId, InstructionSource, Linkage};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub qualified_ty: EagerQualifiedTy,
    pub variant: EagerExprVariant,
    pub instruction_id: InstructionId,
}

impl EagerExpr {
    pub fn ty(&self) -> EntityRoutePtr {
        self.qualified_ty.ty
    }
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }

    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerExprVariant {
    Variable {
        varname: CustomIdentifier,
        binding: Binding,
    },
    ThisValue {
        binding: Binding,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        this_ty: EntityRoutePtr,
        this_binding: Binding,
        field_binding: Binding,
    },
    EntityRoute {
        route: EntityRoutePtr,
    },
    PrimitiveLiteral(CopyableValue),
    EnumKindLiteral(EntityRoutePtr),
    Bracketed(Arc<EagerExpr>),
    Opn {
        opn_variant: EagerOpnVariant,
        opds: Vec<Arc<EagerExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRoutePtr>)>,
        Box<EagerExpr>,
    ),
}

pub fn parse_eager_expr(
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    file: FilePtr,
    raw_expr_idx: RawExprIdx,
) -> SemanticResultArc<EagerExpr> {
    EagerParser::new(db, arena, file).parse_eager_expr(raw_expr_idx)
}
