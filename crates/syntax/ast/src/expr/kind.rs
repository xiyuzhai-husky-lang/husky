use std::sync::Arc;

use atom::AtomKind;

use text::Row;
use vm::PrimitiveValue;
use word::WordPtr;

use crate::*;
use scope::{RangedScope, RawEntityKind, ScopeKind, ScopePtr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RawExprKind {
    Variable {
        varname: CustomIdentifier,
        init_row: Row,
    },
    This {
        ty: Option<ScopePtr>,
    },
    Unrecognized(CustomIdentifier),
    Scope {
        scope: ScopePtr,
        kind: RawEntityKind,
    },
    PrimitiveLiteral(PrimitiveValue),
    Bracketed(RawExprIdx),
    Opn {
        opr: Opr,
        opds: RawExprRange,
    },
    Lambda(Vec<(CustomIdentifier, Option<RangedScope>)>, RawExprIdx),
}
