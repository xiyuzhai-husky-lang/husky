mod error;
mod kind;
mod precedence;
mod stack;

use arena::{map::ArenaMap, Arena, ArenaIdx, ArenaRange};
use atom::Atom;
use entity_route::GenericArgument;
pub use kind::RawExprVariant;
pub(crate) use stack::ExprStack;
pub use word::Keyword;

use crate::*;
use atom::AtomVariant;
use text::TextRange;
use text::TextRanged;
use vm::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RawExpr {
    pub range: TextRange,
    pub variant: RawExprVariant,
}

pub type RawExprMap<V> = ArenaMap<RawExpr, V>;

impl TextRanged for RawExpr {
    fn text_range(&self) -> TextRange {
        self.range
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
    ) -> AstResult<Self> {
        if bracket == Bracket::Par && start_attr == ListStartAttr::None && arena::len(&opds) == 1 {
            return Ok(Self {
                range,
                variant: RawExprVariant::Bracketed(opds.start),
            });
        }
        let opr = match start_attr {
            ListStartAttr::None => match bracket {
                Bracket::Par => ListOpr::TupleInit,
                Bracket::Box => ListOpr::NewVec,
                Bracket::Curl => ListOpr::NewDict,
                Bracket::Angle => todo!(),
            },
            ListStartAttr::Attach => match bracket {
                Bracket::Par => ListOpr::Call,
                Bracket::Box => match end_attr {
                    ListEndAttr::None => {
                        if arena::len(&opds) < 2 {
                            return err!(format!("expect index expr inside `[]`"), range);
                        }
                        ListOpr::Index
                    }
                    ListEndAttr::Modulo => ListOpr::ModuloIndex,
                    ListEndAttr::Attach => todo!(),
                },
                Bracket::Curl => ListOpr::StructInit,
                Bracket::Angle => todo!(),
            },
            ListStartAttr::MethodAttach {
                ranged_ident,
                generic_arguments,
            } => ListOpr::MethodCall {
                ranged_ident,
                generic_arguments,
            },
        }
        .into();
        Ok(Self {
            range,
            variant: RawExprVariant::Opn { opr, opds },
        })
    }

    pub fn opn(range: TextRange, opr: Opr, opds: RawExprRange) -> Self {
        Self {
            range,
            variant: RawExprVariant::Opn { opr, opds },
        }
    }
}

impl From<Atom> for RawExpr {
    fn from(atom: Atom) -> Self {
        Self {
            range: atom.text_range(),
            variant: match atom.kind {
                AtomVariant::Variable {
                    varname,
                    init_range: init_row,
                } => RawExprVariant::Variable {
                    varname,
                    init_range: init_row,
                },
                AtomVariant::PrimitiveLiteral(literal) => {
                    RawExprVariant::PrimitiveLiteral(literal.clone())
                }
                AtomVariant::EntityRoute { route: scope, kind } => {
                    RawExprVariant::Entity { route: scope, kind }
                }
                AtomVariant::ThisData {
                    opt_ty,
                    opt_contract,
                } => RawExprVariant::This {
                    opt_ty,
                    opt_contract,
                },
                AtomVariant::Unrecognized(ident) => RawExprVariant::Unrecognized(ident),
                AtomVariant::FrameVariable {
                    varname,
                    init_range,
                } => RawExprVariant::FrameVariable {
                    varname,
                    init_range,
                },
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
