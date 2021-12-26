mod error;
mod kind;
mod parser;
mod precedence;
mod stack;

use atom::{Bracket, ListEndAttr, ListStartAttr};
use common::*;

use kind::ListOpr;
use text::TextRange;

pub use error::{ExprError, ExprResultArc};
pub use kind::{ExprKind, Opr};
pub use parser::ExprParser;
use text::TextRanged;
pub use word::Keyword;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expr {
    pub range: TextRange,
    pub kind: ExprKind,
}

impl Expr {
    pub fn list(
        bracket: Bracket,
        start_attr: ListStartAttr,
        end_attr: ListEndAttr,
        range: TextRange,
        opds: ExprRange,
    ) -> Self {
        let opr = match start_attr {
            ListStartAttr::None => match bracket {
                Bracket::Par => ListOpr::TupleInit,
                Bracket::Box => ListOpr::NewVec,
                Bracket::Curl => ListOpr::NewDict,
                Bracket::Vert => todo!(),
            },
            ListStartAttr::Attach => match bracket {
                Bracket::Par => ListOpr::Call,
                Bracket::Box => match end_attr {
                    ListEndAttr::None => ListOpr::Index,
                    ListEndAttr::Modulo => ListOpr::ModuloIndex,
                    ListEndAttr::Attach => todo!(),
                },
                Bracket::Curl => ListOpr::StructInit,
                Bracket::Vert => todo!(),
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

pub type ExprResult = Result<Option<(atom::StmtAttr, Option<Expr>)>, ExprError>;

pub type ExprArena = arena::Arena<Expr>;
pub type ExprIdx = arena::ArenaIdx<Expr>;
pub type ExprRange = arena::ArenaRange<Expr>;
