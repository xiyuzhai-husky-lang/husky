mod gen_instructions;
mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
pub(crate) use gen_instructions::ExprInstructionBuilder;
pub use opn::*;
pub(crate) use parser::ExprParser;

use scope::ScopePtr;
use syntax_types::*;
use text::TextRange;
use vm::{Compiled, Contract, InstructionId, InstructionSource, PrimitiveValue, VMResult};
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: ScopePtr,
    pub kind: StrictExprKind,
    pub instruction_id: InstructionId,
    pub contract: Contract,
}

impl InstructionSource for Expr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrictExprKind {
    Variable(CustomIdentifier),
    Scope {
        scope: ScopePtr,
        compiled: Option<Compiled>,
    },
    Literal(PrimitiveValue),
    Bracketed(Arc<Expr>),
    Opn {
        opn: Opn,
        compiled: Option<Compiled>,
        opds: Vec<Arc<Expr>>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopePtr>)>, Box<Expr>),
}
