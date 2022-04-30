mod error;
mod kind;
mod precedence;
mod stack;

use arena::{Arena, ArenaIdx, ArenaRange};
pub use kind::RawExprVariant;
pub(crate) use stack::ExprStack;
pub use word::Keyword;

use crate::*;
use atom::AtomVariant;

use syntax_types::*;
use text::TextRange;
use text::TextRanged;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawExpr {
    pub range: TextRange,
    pub variant: RawExprVariant,
}

impl TextRanged for RawExpr {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}

impl RawExpr {
    pub fn range(&self) -> TextRange {
        self.range
    }

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
                variant: RawExprVariant::Bracketed(opds.start),
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
            variant: RawExprVariant::Opn { opr, opds },
        }
    }

    pub fn opn(range: TextRange, opr: Opr, opds: RawExprRange) -> Self {
        Self {
            range,
            variant: RawExprVariant::Opn { opr, opds },
        }
    }
}

impl From<&atom::Atom> for RawExpr {
    fn from(atom: &atom::Atom) -> Self {
        Self {
            range: atom.text_range(),
            variant: match atom.kind {
                AtomVariant::Variable { varname, init_row } => {
                    RawExprVariant::Variable { varname, init_row }
                }
                AtomVariant::Literal(literal) => RawExprVariant::PrimitiveLiteral(literal.clone()),
                AtomVariant::EntityRoute { route: scope, kind } => {
                    RawExprVariant::Entity { route: scope, kind }
                }
                AtomVariant::ThisData { ty } => RawExprVariant::This { ty },
                AtomVariant::Unrecognized(ident) => RawExprVariant::Unrecognized(ident),
                AtomVariant::FrameVariable { varname, init_row } => {
                    RawExprVariant::FrameVariable { varname, init_row }
                }
                _ => {
                    p!(atom.kind);
                    panic!()
                }
            },
        }
    }
}

pub type RawExprArena = Arena<RawExpr>;
pub type RawExprIdx = ArenaIdx<RawExpr>;
pub type RawExprRange = ArenaRange<RawExpr>;
