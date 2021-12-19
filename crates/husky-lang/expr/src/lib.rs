#![allow(warnings)]
mod error;
mod kind;
mod parser;
mod precedence;
mod query;
mod stack;

use atom::Opr;
use common::*;

use scope::ScopeId;
use text::TextRange;

use error::ExprError;
use kind::{ExprKind, Opn};
use query::ExprQuery;
use word::Keyword;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expr {
    range: TextRange,
    kind: ExprKind,
}

impl Expr {
    pub fn scope_call(range: TextRange, scope_id: ScopeId, opds: ExprRange) -> Self {
        Self {
            range,
            kind: ExprKind::Opn {
                opn: Opn::ScopeCall(scope_id),
                opds,
            },
        }
    }

    pub fn value_call(range: TextRange, opds: ExprRange) -> Self {
        Self {
            range,
            kind: ExprKind::Opn {
                opn: Opn::ValueCall,
                opds,
            },
        }
    }

    pub fn index(range: TextRange, opds: ExprRange) -> Self {
        Self {
            range,
            kind: ExprKind::Opn {
                opn: Opn::Index,
                opds,
            },
        }
    }

    pub fn opn(range: TextRange, opr: Opr, opds: ExprRange) -> Self {
        Self {
            range,
            kind: ExprKind::Opn {
                opn: Opn::Opr(opr),
                opds,
            },
        }
    }
}

impl From<&atom::Atom> for Expr {
    fn from(atom: &atom::Atom) -> Self {
        Self {
            range: atom.range,
            kind: (&atom.kind).into(),
        }
    }
}

pub type ExprResult = Result<(atom::GroupAttr, Expr), ExprError>;

pub type ExprArena = arena::Arena<Expr>;
pub type ExprIdx = arena::ArenaIdx<Expr>;
pub type ExprRange = arena::ArenaRange<Expr>;
