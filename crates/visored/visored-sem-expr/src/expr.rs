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

use self::{attach::AttachDispatch, binary::VdSemBinaryDispatch};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_prelude::script::LxScriptKind;
use literal::VdSemLiteralDispatch;
use visored_opr::opr::binary::VdBaseBinaryOpr;
use visored_zfc_ty::term::literal::VdZfcLiteral;

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum VdSemExprData {
    Literal {
        literal: VdZfcLiteral,
        dispatch: VdSemLiteralDispatch,
    },
    Notation,
    Binary {
        lopd: VdSemExprIdx,
        opr: VdBaseBinaryOpr,
        ropd: VdSemExprIdx,
        dispatch: VdSemBinaryDispatch,
    },
    Prefix {
        opr: VdSemExprIdx,
        opd: VdSemExprIdx,
        dispatch: (),
    },
    Suffix {
        opd: VdSemExprIdx,
        opr: VdSemExprIdx,
        dispatch: (),
    },
    Attach {
        base: VdSemExprIdx,
        // INVARIANCE: at least one of these are some
        scripts: Vec<(LxScriptKind, VdSemExprIdx)>,
        dispatch: AttachDispatch,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
}

pub type VdSemExprIdx = ArenaIdx<VdSemExprData>;
pub type VdSemExprIdxRange = ArenaIdxRange<VdSemExprData>;
pub type VdSemExprArena = Arena<VdSemExprData>;
pub type VdSemExprArenaRef<'a> = ArenaRef<'a, VdSemExprData>;
