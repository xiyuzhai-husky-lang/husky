pub mod attach;
pub mod binary;
pub mod literal;
pub mod notation;
pub mod prefix;
pub mod suffix;
pub mod uniadic_array;
pub mod uniadic_chain;
pub mod variadic_array;
pub mod variadic_chain;

use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_prelude::script::LxScriptKind;
use visored_opr::opr::binary::VdBinaryOpr;
use visored_zfs_ty::term::literal::VdZfsLiteral;

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSynExprData {
    Literal {
        literal: VdZfsLiteral,
    },
    Notation,
    Binary {
        lopd: VdSynExprIdx,
        opr: VdBinaryOpr,
        ropd: VdSynExprIdx,
    },
    Prefix {
        opr: VdSynExprIdx,
        opd: VdSynExprIdx,
    },
    Suffix {
        opd: VdSynExprIdx,
        opr: VdSynExprIdx,
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
            VdSynExprData::Binary { lopd, opr, ropd } => vec![lopd, ropd],
            VdSynExprData::Prefix { opr, opd } => vec![opr, opd],
            VdSynExprData::Suffix { opd, opr } => vec![opd, opr],
            VdSynExprData::Attach { base, ref scripts } => [base]
                .into_iter()
                .chain(scripts.iter().map(|&(_, script)| script))
                .collect(),
            VdSynExprData::UniadicChain => vec![],
            VdSynExprData::VariadicChain => vec![],
            VdSynExprData::UniadicArray => vec![],
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
