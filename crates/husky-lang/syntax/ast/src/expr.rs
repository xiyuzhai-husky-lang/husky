mod error;
mod kind;
mod precedence;
mod stack;

pub use kind::RawExprKind;
pub(crate) use stack::ExprStack;
pub use word::Keyword;

use crate::atom::AtomKind;
use crate::*;
use common::*;
use syntax_types::*;
use text::TextRange;
use text::TextRanged;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawExpr {
    pub range: TextRange,
    pub kind: RawExprKind,
}

impl RawExpr {
    pub fn synthesize_list(
        bracket: Bracket,
        start_attr: ListStartAttr,
        end_attr: ListEndAttr,
        range: TextRange,
        opds: RawExprRange,
    ) -> Self {
        if bracket == Bracket::Par && start_attr == ListStartAttr::None && arena::len(&opds) == 1 {
            return Self {
                range,
                kind: RawExprKind::Bracketed(opds.start),
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
            kind: RawExprKind::Opn { opr, opds },
        }
    }

    pub fn opn(range: TextRange, opr: Opr, opds: RawExprRange) -> Self {
        Self {
            range,
            kind: RawExprKind::Opn { opr, opds },
        }
    }
}

impl From<&atom::Atom> for RawExpr {
    fn from(atom: &atom::Atom) -> Self {
        Self {
            range: atom.text_range(),
            kind: match atom.kind {
                AtomKind::Variable(ident) => RawExprKind::Variable(ident),
                AtomKind::Literal(literal) => RawExprKind::Literal(literal.clone()),
                AtomKind::Scope { scope, kind } => RawExprKind::Scope { scope, kind },
                _ => {
                    p!(atom.kind);
                    panic!()
                }
            },
        }
    }
}

pub type RawExprArena = arena::Arena<RawExpr>;
pub type RawExprIdx = arena::ArenaIdx<RawExpr>;
pub type RawExprRange = arena::ArenaRange<RawExpr>;
