mod opn;
mod parser;
mod xml;

use husky_pattern_semantics::PurePattern;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
pub use xml::*;

use std::sync::Arc;

use husky_file::FilePtr;
use husky_infer_qualified_ty::LazyValueQualifiedTy;
pub use opn::*;
pub(crate) use parser::LazyExprParser;

use husky_entity_route::{EntityRoute, EntityRoutePtr, RangedEntityRoute};
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_vm::*;
use husky_word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub qualified_ty: LazyValueQualifiedTy,
    pub variant: LazyExprVariant,
    pub instruction_id: InstructionId,
}

impl LazyExpr {
    pub fn ty(&self) -> EntityRoutePtr {
        self.qualified_ty.ty
    }
}

impl InstructionSource for LazyExpr {
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

#[derive(Clone, PartialEq, Eq)]
pub enum LazyExprVariant {
    Variable {
        varname: CustomIdentifier,
        binding: Binding,
    },
    PrimitiveLiteral(PrimitiveLiteralData),
    EnumLiteral {
        entity_route: EntityRoutePtr,
    },
    Bracketed(Arc<LazyExpr>),
    Opn {
        opn_kind: LazyOpnKind,
        opds: Vec<Arc<LazyExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRoutePtr>)>,
        Box<LazyExpr>,
    ),
    ThisValue {
        binding: Binding,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        this_ty: EntityRoutePtr,
        this_binding: Binding,
        field_binding: Binding,
    },
    EntityFeature {
        entity_route: EntityRoutePtr,
    },
    BePattern {
        this: Arc<LazyExpr>,
        patt: Arc<PurePattern>,
    },
}

impl std::fmt::Debug for LazyExprVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable { varname, binding } => f
                .debug_struct("Variable")
                .field("varname", varname)
                .field("binding", binding)
                .finish(),
            Self::PrimitiveLiteral(arg0) => f.debug_tuple("PrimitiveLiteral").field(arg0).finish(),
            Self::EnumLiteral { entity_route } => f
                .debug_struct("EnumLiteral")
                .field("entity_route", entity_route)
                .finish(),
            Self::Bracketed(arg0) => f.write_str("Bracketed"),
            Self::Opn { opn_kind, opds } => {
                f.debug_struct("Opn").field("opn_kind", opn_kind).finish()
            }
            Self::Lambda(arg0, arg1) => f.debug_tuple("Lambda").field(arg0).field(arg1).finish(),
            Self::ThisValue { binding } => f
                .debug_struct("ThisValue")
                .field("binding", binding)
                .finish(),
            Self::ThisField {
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
            Self::EntityFeature { entity_route } => f
                .debug_struct("EntityFeature")
                .field("entity_route", entity_route)
                .finish(),
            LazyExprVariant::Variable { varname, binding } => todo!(),
            LazyExprVariant::PrimitiveLiteral(_) => todo!(),
            LazyExprVariant::EnumLiteral { entity_route } => todo!(),
            LazyExprVariant::Bracketed(_) => todo!(),
            LazyExprVariant::Opn { opn_kind, opds } => todo!(),
            LazyExprVariant::Lambda(_, _) => todo!(),
            LazyExprVariant::ThisValue { binding } => todo!(),
            LazyExprVariant::ThisField {
                field_ident,
                this_ty,
                this_binding,
                field_binding,
            } => todo!(),
            LazyExprVariant::EntityFeature { entity_route } => todo!(),
            LazyExprVariant::BePattern { .. } => todo!(),
        }
    }
}
