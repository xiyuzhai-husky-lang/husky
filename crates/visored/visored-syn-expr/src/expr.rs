pub mod attach;
pub mod binary;
pub mod list_item;
pub mod literal;
pub mod notation;
pub mod prefix;
pub mod suffix;
pub mod uniadic_array;
pub mod uniadic_chain;
pub mod variadic_array;
pub mod variadic_chain;

use either::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_prelude::script::LxScriptKind;
use visored_opr::opr::{binary::VdBinaryOpr, prefix::VdPrefixOpr, suffix::VdSuffixOpr, VdOpr};
use visored_zfs_ty::term::literal::VdZfsLiteral;

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSynExprData {
    Literal {
        literal: VdZfsLiteral,
    },
    Notation,
    Opr {
        opr: VdOpr,
    },
    Binary {
        lopd: VdSynExprIdx,
        opr: Either<VdBinaryOpr, VdSynExprIdx>,
        ropd: VdSynExprIdx,
    },
    Prefix {
        opr: Either<VdPrefixOpr, VdSynExprIdx>,
        opd: VdSynExprIdx,
    },
    Suffix {
        opd: VdSynExprIdx,
        opr: Either<VdSuffixOpr, VdSynExprIdx>,
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
}

pub type VdSynExprIdx = ArenaIdx<VdSynExprData>;
pub type VdSynExprIdxRange = ArenaIdxRange<VdSynExprData>;
pub type VdSynExprArena = Arena<VdSynExprData>;
pub type VdSynExprArenaRef<'a> = ArenaRef<'a, VdSynExprData>;

impl VdSynExprData {
    pub fn children(&self) -> Vec<VdSynExprIdx> {
        match *self {
            VdSynExprData::Literal { literal } => vec![],
            VdSynExprData::Notation => vec![],
            VdSynExprData::Opr { opr } => vec![],
            VdSynExprData::Binary { lopd, opr, ropd } => match opr {
                Left(_) => vec![lopd, ropd],
                Right(opr) => vec![lopd, opr, ropd],
            },
            VdSynExprData::Prefix { opr, opd } => match opr {
                Left(_) => vec![opd],
                Right(opr) => vec![opr, opd],
            },
            VdSynExprData::Suffix { opd, opr } => match opr {
                Left(_) => vec![opd],
                Right(opr) => vec![opd, opr],
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
        }
    }

    pub fn class(&self) -> VdSynExprClass {
        match *self {
            // ad hoc
            _ => VdSynExprClass::Separator,
            _ => unreachable!(),
        }
    }
}

pub enum VdSynExprClass {
    Prefix,
    Suffix,
    Separator,
}
