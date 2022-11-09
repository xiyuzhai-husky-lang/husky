mod opn;
mod parser;
mod xml;

use husky_opn_semantics::ImplicitConversion;
use husky_pattern_semantics::PurePattern;
use husky_primitive_literal_syntax::RawLiteralData;
pub use xml::*;

use std::sync::Arc;

use husky_file::FileItd;
pub use opn::*;
pub(crate) use parser::LazyExprParser;

use husky_entity_route::EntityRouteItd;
use husky_text::{FileRanged, RangedCustomIdentifier, TextRange, TextRanged};
use husky_vm::*;
use husky_word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExpr {
    pub file: FileItd,
    pub range: TextRange,
    pub qualified_ty: (),
    pub variant: LazyExprVariant,
    pub instruction_id: InstructionId,
    pub implicit_conversion: ImplicitConversion,
}

impl TextRanged for LazyExpr {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl FileRanged for LazyExpr {
    fn file(&self) -> FileItd {
        self.file
    }
}

impl LazyExpr {
    pub fn intrinsic_ty(&self) -> EntityRouteItd {
        todo!()
        // self.qualified_ty.intrinsic_ty()
    }
}

impl InstructionSource for LazyExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum LazyExprVariant {
    Variable {
        varname: CustomIdentifier,
        binding: Binding,
    },
    PrimitiveLiteral(RawLiteralData),
    EnumLiteral {
        entity_route: EntityRouteItd,
    },
    Bracketed(Arc<LazyExpr>),
    Opn {
        opn_kind: LazyOpnKind,
        opds: Vec<Arc<LazyExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRouteItd>)>,
        Box<LazyExpr>,
    ),
    ThisValue {
        binding: Binding,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        this_ty: EntityRouteItd,
        this_binding: Binding,
        field_binding: Binding,
    },
    EntityFeature {
        entity_route: EntityRouteItd,
    },
    BePattern {
        this: Arc<LazyExpr>,
        patt: Arc<PurePattern>,
    },
}

impl std::fmt::Debug for LazyExprVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LazyExprVariant::Variable { varname, binding } => f
                .debug_struct("Variable")
                .field("varname", varname)
                .field("binding", binding)
                .finish(),
            LazyExprVariant::PrimitiveLiteral(arg0) => {
                f.debug_tuple("PrimitiveLiteral").field(arg0).finish()
            }
            LazyExprVariant::EnumLiteral { entity_route } => f
                .debug_struct("EnumLiteral")
                .field("entity_route", entity_route)
                .finish(),
            LazyExprVariant::Bracketed(_) => f.write_str("Bracketed"),
            LazyExprVariant::Opn { opn_kind, .. } => {
                f.debug_struct("Opn").field("opn_kind", opn_kind).finish()
            }
            LazyExprVariant::Lambda(arg0, arg1) => {
                f.debug_tuple("Lambda").field(arg0).field(arg1).finish()
            }
            LazyExprVariant::ThisValue { binding } => f
                .debug_struct("ThisValue")
                .field("binding", binding)
                .finish(),
            LazyExprVariant::ThisField {
                field_ident,
                this_ty,
                this_binding,
                field_binding,
            } => f
                .debug_struct("ThisField")
                .field("field_ident", field_ident)
                .field("this_ty", this_ty)
                .field("this_binding", this_binding)
                .field("field_binding", field_binding)
                .finish(),
            LazyExprVariant::EntityFeature { entity_route } => f
                .debug_struct("EntityFeature")
                .field("entity_route", entity_route)
                .finish(),
            LazyExprVariant::BePattern { .. } => f.debug_struct("BePattern").finish(),
        }
    }
}
