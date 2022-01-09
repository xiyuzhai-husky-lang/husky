mod error;
mod kind;
mod precedence;
mod stack;

pub use kind::ListOpr;
pub use kind::{ExprKind, Opr};
pub(crate) use stack::ExprStack;
pub use word::Keyword;

use crate::*;
use atom::{Bracket, ListEndAttr, ListStartAttr};
use common::*;
use text::TextRange;
use text::TextRanged;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expr {
    pub range: TextRange,
    pub kind: ExprKind,
}

impl Expr {
    pub fn synthesize_list(
        bracket: Bracket,
        start_attr: ListStartAttr,
        end_attr: ListEndAttr,
        range: TextRange,
        opds: ExprRange,
    ) -> Self {
        if bracket == Bracket::Par && start_attr == ListStartAttr::None && arena::len(&opds) == 1 {
            return Self {
                range,
                kind: ExprKind::Bracketed(opds.start),
            };
        }
        let opr = match start_attr {
            ListStartAttr::None => match bracket {
                Bracket::Par => ListOpr::TupleInit,
                Bracket::Box => ListOpr::NewVec,
                Bracket::Curl => ListOpr::NewDict,
            },
            ListStartAttr::Attach => match bracket {
                Bracket::Par => ListOpr::Call,
                Bracket::Box => match end_attr {
                    ListEndAttr::None => ListOpr::Index,
                    ListEndAttr::Modulo => ListOpr::ModuloIndex,
                    ListEndAttr::Attach => todo!(),
                },
                Bracket::Curl => ListOpr::StructInit,
            },
        }
        .into();
        Self {
            range,
            kind: ExprKind::Opn { opr, opds },
        }
    }

    pub fn opn(range: TextRange, opr: Opr, opds: ExprRange) -> Self {
        Self {
            range,
            kind: ExprKind::Opn { opr, opds },
        }
    }
}

impl From<&atom::Atom> for Expr {
    fn from(atom: &atom::Atom) -> Self {
        Self {
            range: atom.text_range(),
            kind: (&atom.kind).into(),
        }
    }
}

pub type ExprArena = arena::Arena<Expr>;
pub type ExprIdx = arena::ArenaIdx<Expr>;
pub type ExprRange = arena::ArenaRange<Expr>;
