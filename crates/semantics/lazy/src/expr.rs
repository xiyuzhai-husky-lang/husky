mod opn;
mod parser;

use std::sync::Arc;

use file::FilePtr;
pub use opn::*;
pub(crate) use parser::LazyExprParser;

use scope::{RangedScope, Scope, ScopePtr};
use syntax_types::*;
use text::TextRange;
use vm::*;
use word::{CustomIdentifier, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub ty: ScopePtr,
    pub kind: LazyExprKind,
    pub instruction_id: InstructionId,
    pub contract: LazyContract,
}

impl InstructionSource for LazyExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LazyExprKind {
    Variable(CustomIdentifier),
    Scope {
        scope: ScopePtr,
        compiled: (),
    },
    PrimitiveLiteral(PrimitiveValue),
    EnumLiteral {
        scope: ScopePtr,
        value: EnumLiteralValue,
    },
    Bracketed(Arc<LazyExpr>),
    Opn {
        opn_kind: LazyOpnKind,
        compiled: (),
        opds: Vec<Arc<LazyExpr>>,
    },
    Lambda(Vec<(CustomIdentifier, Option<ScopePtr>)>, Box<LazyExpr>),
    This,
    ScopedFeature {
        scope: ScopePtr,
    },
}
