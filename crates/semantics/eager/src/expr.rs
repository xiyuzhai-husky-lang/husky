mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
use infer_qualifier::{EagerQualifiedType, EagerQualifier};
pub use opn::*;
pub(crate) use parser::EagerExprParser;

use entity_route::EntityRoutePtr;
use text::TextRange;
use vm::{EagerContract, InstructionId, InstructionSource, Linkage, PrimitiveValue};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub qualified_ty: EagerQualifiedType,
    pub ty: EntityRoutePtr,
    pub variant: EagerExprVariant,
    pub instruction_id: InstructionId,
    pub contract: EagerContract,
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerExprVariant {
    Variable(CustomIdentifier),
    This,
    EntityRoute {
        route: EntityRoutePtr,
    },
    PrimitiveLiteral(PrimitiveValue),
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
