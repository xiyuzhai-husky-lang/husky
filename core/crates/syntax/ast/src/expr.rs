mod error;
mod precedence;
mod stack;
mod variant;

pub use variant::*;
pub use word::Keyword;

use crate::*;
use arena::{map::ArenaMap, Arena, ArenaIdx, ArenaRange};
use husky_atom::AtomVariant;
use husky_atom::HuskyAtom;
use husky_text::TextRange;
use husky_text::TextRanged;
pub(crate) use stack::ExprStack;
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
        let opn_variant = RawOpnVariant::List(match start_attr {
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
        });
        Ok(Self {
            range,
            variant: RawExprVariant::Opn { opn_variant, opds },
        })
    }

    pub fn opn(range: TextRange, opn_variant: RawOpnVariant, opds: RawExprRange) -> Self {
        Self {
            range,
            variant: RawExprVariant::Opn { opn_variant, opds },
        }
    }
}

impl From<HuskyAtom> for RawExpr {
    fn from(atom: HuskyAtom) -> Self {
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
                    RawExprVariant::CopyableLiteral(literal.clone())
                }
                AtomVariant::EntityRoute { route: scope, kind } => {
                    RawExprVariant::Entity { route: scope, kind }
                }
                AtomVariant::ThisValue {
                    opt_this_ty,
                    opt_this_liason,
                } => RawExprVariant::ThisValue {
                    opt_this_ty,
                    opt_this_liason,
                },
                AtomVariant::ThisField {
                    opt_this_ty,
                    opt_this_liason,
                    field_ident,
                    field_liason,
                    opt_field_ty,
                } => RawExprVariant::ThisField {
                    opt_this_ty,
                    opt_this_liason,
                    field_ident,
                    opt_field_ty,
                    field_liason,
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
