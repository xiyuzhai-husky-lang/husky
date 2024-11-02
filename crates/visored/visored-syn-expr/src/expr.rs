pub mod attach;
pub mod binary;
pub mod list_item;
pub mod literal;
pub mod notation;
pub mod prefix;
pub mod suffix;
#[cfg(test)]
mod tests;
pub mod uniadic_array;
pub mod uniadic_chain;
pub mod variadic_array;
pub mod variadic_chain;

use std::fmt::Formatter;

use crate::builder::{ToVdSyn, VdSynExprBuilder};
use crate::*;
use either::*;
use error::{OriginalVdSynExprError, VdSynExprError};
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::math::{LxMathAstIdx, LxMathAstIdxRange};
use latex_math_letter::LxMathLetter;
use latex_prelude::script::LxScriptKind;
use latex_token::idx::{LxMathTokenIdx, LxTokenIdx, LxTokenIdxRange};
use range::VdSynExprTokenIdxRange;
use visored_opr::{
    delimiter::{
        VdBaseLeftDelimiter, VdBaseRightDelimiter, VdCompositeLeftDelimiter,
        VdCompositeRightDelimiter,
    },
    opr::{
        binary::{VdBaseBinaryOpr, VdCompositeBinaryOpr},
        prefix::{VdBasePrefixOpr, VdCompositePrefixOpr},
        suffix::{VdBaseSuffixOpr, VdCompositeSuffixOpr},
        VdBaseOpr,
    },
    precedence::VdPrecedenceRange,
    separator::{VdBaseSeparator, VdCompositeSeparator, VdSeparator},
};
use visored_zfc_ty::term::literal::{VdZfcLiteral, VdZfcLiteralData};

/// It's a tree of both form and meaning
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum VdSynExprData {
    Literal {
        token_idx_range: LxTokenIdxRange,
        literal: VdZfcLiteral,
    },
    Notation,
    Letter {
        token_idx: LxMathTokenIdx,
        letter: LxMathLetter,
    },
    BaseOpr {
        opr: VdBaseOpr,
    },
    Binary {
        lopd: VdSynExprIdx,
        opr: VdSynBinaryOpr,
        ropd: VdSynExprIdx,
    },
    Prefix {
        opr: VdSynPrefixOpr,
        opd: VdSynExprIdx,
    },
    Suffix {
        opd: VdSynExprIdx,
        opr: VdSynSuffixOpr,
    },
    SeparatedList {
        separator: VdSeparator,
        fragments: SmallVec<[Either<VdSynExprIdx, VdSynSeparator>; 4]>,
    },
    Attach {
        base: VdSynExprIdx,
        // INVARIANCE: at least one of these are some
        scripts: Vec<(LxScriptKind, VdSynExprIdx)>,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
    Err(VdSynExprError),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynPrefixOpr {
    Base(LxTokenIdxRange, VdBasePrefixOpr),
    Composite(VdSynExprIdx, VdCompositePrefixOpr),
}

impl VdSynPrefixOpr {
    pub(crate) fn show(&self, db: &::salsa::Db, arena: VdSynExprArenaRef) -> String {
        match *self {
            VdSynPrefixOpr::Base(_, opr) => opr.latex_code().to_string(),
            VdSynPrefixOpr::Composite(_, opr) => opr.latex_code().to_string(), // ad hoc
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynSuffixOpr {
    Base(LxTokenIdxRange, VdBaseSuffixOpr),
    Composite(VdSynExprIdx, VdCompositeSuffixOpr),
}

impl VdSynSuffixOpr {
    pub(crate) fn show(&self, arena: VdSynExprArenaRef) -> String {
        match *self {
            VdSynSuffixOpr::Base(_, opr) => opr.latex_code().to_string(),
            VdSynSuffixOpr::Composite(_, opr) => opr.latex_code().to_string(), // ad hoc
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynBinaryOpr {
    Base(LxTokenIdxRange, VdBaseBinaryOpr),
    Composite(VdSynExprIdx, VdCompositeBinaryOpr),
}

impl VdSynBinaryOpr {
    pub(crate) fn left_precedence_range(self) -> VdPrecedenceRange {
        match self {
            VdSynBinaryOpr::Base(_, opr) => opr.left_precedence_range(),
            VdSynBinaryOpr::Composite(_, opr) => opr.left_precedence_range(),
        }
    }

    pub(crate) fn precedence(self) -> visored_opr::precedence::VdPrecedence {
        match self {
            VdSynBinaryOpr::Base(_, opr) => opr.precedence(),
            VdSynBinaryOpr::Composite(_, opr) => opr.precedence(),
        }
    }
}

impl VdSynBinaryOpr {
    pub(crate) fn show(&self, db: &::salsa::Db, arena: VdSynExprArenaRef) -> String {
        match *self {
            VdSynBinaryOpr::Base(_, opr) => opr.latex_code().to_string(),
            VdSynBinaryOpr::Composite(_, opr) => opr.latex_code().to_string(), // ad hoc
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynSeparator {
    Base(LxMathTokenIdx, VdBaseSeparator),
    Composite(VdSynExprIdx, VdCompositeSeparator),
}

impl VdSynSeparator {
    pub(crate) fn show(&self, db: &::salsa::Db, arena: VdSynExprArenaRef) -> String {
        match *self {
            VdSynSeparator::Base(_, slf) => slf.latex_code().to_string(),
            VdSynSeparator::Composite(slf, _) => arena[slf].show(db, arena),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynLeftDelimiter {
    Base(VdBaseLeftDelimiter),
    Composite(VdSynExprIdx, VdCompositeLeftDelimiter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynRightDelimiter {
    Base(VdBaseRightDelimiter),
    Composite(VdSynExprIdx, VdCompositeRightDelimiter),
}

pub type VdSynExprIdx = ArenaIdx<VdSynExprData>;
pub type VdSynExprIdxRange = ArenaIdxRange<VdSynExprData>;
pub type VdSynExprArena = Arena<VdSynExprData>;
pub type VdSynExprMap<T> = ArenaMap<VdSynExprData, T>;
pub type VdSynExprOrderedMap<T> = ArenaOrderedMap<VdSynExprData, T>;
pub type VdSynExprArenaRef<'a> = ArenaRef<'a, VdSynExprData>;

impl VdSynExprData {
    pub fn children(&self) -> Vec<VdSynExprIdx> {
        match *self {
            VdSynExprData::Literal { .. }
            | VdSynExprData::Notation
            | VdSynExprData::Letter { .. }
            | VdSynExprData::BaseOpr { .. } => vec![],
            VdSynExprData::Binary { lopd, opr, ropd } => match opr {
                VdSynBinaryOpr::Base(_, _) => vec![lopd, ropd],
                VdSynBinaryOpr::Composite(opr, _) => vec![lopd, opr, ropd],
            },
            VdSynExprData::Prefix { opr, opd } => match opr {
                VdSynPrefixOpr::Base(_, _) => vec![opd],
                VdSynPrefixOpr::Composite(opr, _) => vec![opr, opd],
            },
            VdSynExprData::Suffix { opd, opr } => match opr {
                VdSynSuffixOpr::Base(_, _) => vec![opd],
                VdSynSuffixOpr::Composite(opr, _) => vec![opd, opr],
            },
            VdSynExprData::Attach { base, ref scripts } => [base]
                .into_iter()
                .chain(scripts.iter().map(|&(_, script)| script))
                .collect(),
            // ad hoc
            VdSynExprData::UniadicChain => vec![],
            // ad hoc
            VdSynExprData::VariadicChain => vec![],
            // ad hoc
            VdSynExprData::UniadicArray => vec![],
            // ad hoc
            VdSynExprData::VariadicArray => vec![],
            VdSynExprData::Err(..) => vec![],
            VdSynExprData::SeparatedList { ref fragments, .. } => fragments
                .iter()
                .filter_map(|fragment| match *fragment {
                    Left(expr) | Right(VdSynSeparator::Composite(expr, _)) => Some(expr),
                    Right(VdSynSeparator::Base(_, _)) => None,
                })
                .collect(),
        }
    }

    #[track_caller]
    pub fn class(&self) -> VdSynExprClass {
        match *self {
            VdSynExprData::Literal { .. }
            | VdSynExprData::Notation
            | VdSynExprData::Letter { .. }
            | VdSynExprData::BaseOpr { .. } => VdSynExprClass::Atom,
            VdSynExprData::Binary { .. } => todo!(),
            VdSynExprData::Prefix { .. } => todo!(),
            VdSynExprData::Suffix { .. } => todo!(),
            VdSynExprData::Attach { .. } => todo!(),
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(..) => todo!(),
            VdSynExprData::SeparatedList { .. } => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynExprClass {
    Atom,
    Prefix,
    Suffix,
    Separator,
}

// token idx range is needed because the ast idx range might be empty,
// in which case we need to return an error yet can't determine the token idx range from the ast idx range alone
impl ToVdSyn<VdSynExprIdx> for (LxTokenIdxRange, LxMathAstIdxRange) {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynExprIdx {
        let (token_range, asts) = self;
        if asts.is_empty() {
            builder.alloc_expr(VdSynExprData::Err(
                OriginalVdSynExprError::Empty(token_range).into(),
            ))
        } else {
            let parser = builder.parser();
            parser.parse_asts(asts)
        }
    }
}

impl ToVdSyn<VdSynExprIdx> for LxMathAstIdx {
    fn to_vd_syn(self, builder: &mut VdSynExprBuilder) -> VdSynExprIdx {
        todo!()
    }
}

impl VdSynExprData {
    pub fn show(&self, db: &::salsa::Db, arena: VdSynExprArenaRef) -> String {
        match *self {
            VdSynExprData::Literal {
                token_idx_range,
                literal,
            } => match literal.data(db) {
                VdZfcLiteralData::NaturalNumber(n) => {
                    debug_assert!(n.is_empty());
                    n.to_string()
                }
                VdZfcLiteralData::NegativeInteger(n) => n.to_string(),
                VdZfcLiteralData::FiniteDecimalRepresentation(n) => n.to_string(),
                VdZfcLiteralData::SpecialConstant(vd_zfc_special_constant) => todo!(),
            },
            VdSynExprData::Notation => todo!(),
            VdSynExprData::Letter { token_idx, letter } => letter.latex_code().to_string(),
            VdSynExprData::BaseOpr { opr } => opr.latex_code().to_string(),
            VdSynExprData::Binary { lopd, opr, ropd } => {
                format!(
                    "{}%{}&{}",
                    arena[lopd].show(db, arena),
                    opr.show(db, arena),
                    arena[ropd].show(db, arena)
                )
            }
            VdSynExprData::Prefix { opr, opd } => todo!(),
            VdSynExprData::Suffix { opd, opr } => todo!(),
            VdSynExprData::SeparatedList {
                separator,
                ref fragments,
            } => todo!(),
            VdSynExprData::Attach { base, ref scripts } => todo!(),
            VdSynExprData::UniadicChain => todo!(),
            VdSynExprData::VariadicChain => todo!(),
            VdSynExprData::UniadicArray => todo!(),
            VdSynExprData::VariadicArray => todo!(),
            VdSynExprData::Err(ref error) => error.to_string(),
        }
    }
}
