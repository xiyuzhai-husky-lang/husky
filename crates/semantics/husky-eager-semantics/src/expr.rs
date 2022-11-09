mod opn;
mod parser;

use crate::*;
use husky_expr_syntax::*;
use husky_file::FileItd;
use husky_primitive_literal_syntax::RawLiteralData;
use husky_term_infer::TermInferDb;
pub use opn::*;
pub(crate) use parser::ParseEagerExpr;
use std::sync::Arc;

use husky_entity_route::EntityRouteItd;
use husky_semantics_error::SemanticResultArc;
use husky_text::{FileRanged, RangedCustomIdentifier, TextRange, TextRanged};
use husky_vm::{Binding, InstructionId, InstructionSource};
use husky_word::CustomIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FileItd,
    pub range: TextRange,
    pub qualified_ty: (),
    pub implicit_conversion: ImplicitConversion,
    pub variant: EagerExprVariant,
    pub instruction_id: InstructionId,
}

impl std::fmt::Debug for EagerExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EagerExpr")
            .field("file", &self.file)
            .field("range", &self.range)
            .field("qualified_ty", &self.qualified_ty)
            .field("instruction_id", &self.instruction_id)
            .field("variant", &self.variant)
            .finish()
    }
}

impl EagerExpr {
    pub fn intrinsic_ty(&self) -> EntityRouteItd {
        todo!()
        // self.qualified_ty.intrinsic_ty()
    }
}

impl TextRanged for EagerExpr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
impl FileRanged for EagerExpr {
    fn file(&self) -> FileItd {
        self.file
    }
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Clone, PartialEq, Eq)]
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
        field_idx: u8,
        this_ty: EntityRouteItd,
        this_binding: Binding,
        field_binding: Binding,
    },
    PrimitiveLiteral(RawLiteralData),
    EnumKindLiteral(EntityRouteItd),
    Bracketed(Arc<EagerExpr>),
    Opn {
        opn_variant: EagerOpnVariant,
        opds: Vec<Arc<EagerExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRouteItd>)>,
        Box<EagerExpr>,
    ),
    EntityThickFp {
        route: EntityRouteItd,
    },
    EntityFeature {
        route: EntityRouteItd,
    },
}

impl std::fmt::Debug for EagerExprVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable { varname, binding } => f
                .debug_struct("Variable")
                .field("varname", varname)
                .field("binding", binding)
                .finish(),
            Self::ThisValue { binding } => f
                .debug_struct("ThisValue")
                .field("binding", binding)
                .finish(),
            Self::ThisField {
                field_ident,
                field_idx,
                this_ty,
                this_binding,
                field_binding,
            } => f
                .debug_struct("ThisField")
                .field("field_ident", field_ident)
                .field("field_idx", field_idx)
                .field("this_ty", this_ty)
                .field("this_binding", this_binding)
                .field("field_binding", field_binding)
                .finish(),
            Self::PrimitiveLiteral(arg0) => f.debug_tuple("PrimitiveLiteral").field(arg0).finish(),
            Self::EnumKindLiteral(arg0) => f.debug_tuple("EnumKindLiteral").field(arg0).finish(),
            Self::Bracketed(_) => f.write_str("Bracketed"),
            Self::Opn { opn_variant, .. } => f
                .debug_struct("Opn")
                .field("opn_variant", opn_variant)
                .finish(),
            Self::Lambda(arg0, arg1) => f.debug_tuple("Lambda").field(arg0).field(arg1).finish(),
            Self::EntityThickFp { route } => {
                f.debug_struct("EntityFp").field("route", route).finish()
            }
            Self::EntityFeature { route } => f
                .debug_struct("EntityFeature")
                .field("route", route)
                .finish(),
        }
    }
}

pub fn parse_eager_expr(
    db: &dyn TermInferDb,
    arena: &RawExprArena,
    file: FileItd,
    raw_expr_idx: RawExprIdx,
) -> SemanticResultArc<EagerExpr> {
    todo!()
    // EagerParser::new(db, arena, file).parse_eager_expr(raw_expr_idx, None)
}
