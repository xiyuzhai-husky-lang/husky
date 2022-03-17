mod gen_instructions;
mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
pub(crate) use gen_instructions::ExprInstructionBuilder;
pub use opn::*;
pub(crate) use parser::EagerExprParser;

use scope::ScopePtr;
use syntax_types::*;
use text::TextRange;
use vm::{Compiled, InputContract, InstructionId, InstructionSource, PrimitiveValue, VMResult};
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: ScopePtr,
    pub kind: EagerExprKind,
    pub instruction_id: InstructionId,
    pub contract: InputContract,
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerExprKind {
    Variable(CustomIdentifier),
    Scope {
        scope: ScopePtr,
        compiled: Option<Compiled>,
    },
    Literal(PrimitiveValue),
    Bracketed(Arc<EagerExpr>),
    Opn {
        opn_kind: EagerOpnKind,
        compiled: Option<Compiled>,
        opds: Vec<Arc<EagerExpr>>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopePtr>)>, Box<EagerExpr>),
}
