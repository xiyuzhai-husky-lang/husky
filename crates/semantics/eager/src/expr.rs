mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
use infer_qualifier::{EagerQualifiedTy, EagerQualifier};
pub use opn::*;
pub(crate) use parser::EagerExprParser;

use entity_route::EntityRoutePtr;
use text::TextRange;
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
    ThisData {
        binding: Binding,
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
