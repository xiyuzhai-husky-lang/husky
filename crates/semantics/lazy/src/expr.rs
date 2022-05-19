mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
use infer_qualifier::LazyQualifiedTy;
pub use opn::*;
pub(crate) use parser::LazyExprParser;

use entity_route::{EntityRoute, EntityRoutePtr, RangedEntityRoute};
use syntax_types::*;
use text::TextRange;
use vm::*;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub qualified_ty: LazyQualifiedTy,
    pub variant: LazyExprVariant,
    pub instruction_id: InstructionId,
    pub contract: LazyContract,
}

impl LazyExpr {
    pub fn binding(&self) -> Binding {
        self.qualified_ty.qual.binding(self.contract)
    }

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
    Variable(CustomIdentifier),
    Scope {
        scope: EntityRoutePtr,
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
    This,
    EntityFeature {
        route: EntityRoutePtr,
    },
}
