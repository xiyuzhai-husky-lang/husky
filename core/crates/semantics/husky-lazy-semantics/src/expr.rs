mod opn;
mod parser;
mod xml;

pub use xml::*;

use std::sync::Arc;

use file::FilePtr;
use infer_qualifier::LazyValueQualifiedTy;
pub use opn::*;
pub(crate) use parser::LazyExprParser;

use husky_entity_route_syntax::{EntityRoute, EntityRoutePtr, RangedEntityRoute};
use husky_text::{RangedCustomIdentifier, TextRange};
use vm::*;
use word::{CustomIdentifier, Identifier};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyExprVariant {
    Variable {
        varname: CustomIdentifier,
        binding: Binding,
    },
    EntityRoute {
        route: EntityRoutePtr,
        compiled: (),
    },
    PrimitiveLiteral(CopyableValue),
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
}
