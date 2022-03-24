mod gen_instructions;
mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
pub use opn::*;
pub(crate) use parser::EagerExprParser;

use scope::ScopePtr;
use text::TextRange;
use vm::{Compiled, EagerContract, InstructionId, InstructionSource, PrimitiveValue};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: ScopePtr,
    pub kind: EagerExprKind,
    pub instruction_id: InstructionId,
    pub contract: EagerContract,
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerExprKind {
    Variable(CustomIdentifier),
    This,
    Scope {
        scope: ScopePtr,
        compiled: Option<Compiled>,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(Arc<EagerExpr>),
    Opn {
        opn_kind: EagerOpnKind,
        compiled: Option<Compiled>,
        opds: Vec<Arc<EagerExpr>>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopePtr>)>, Box<EagerExpr>),
}
