mod attach;
mod notation;
mod prefix;
mod suffix;
mod uniadic_array;
mod uniadic_chain;
mod variadic_array;
mod variadic_chain;

use attach::AttachDispatch;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum SemExprData {
    Notation,
    Binary {
        opr: (),
        dispatch: (),
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
        top: Option<VdSemExprIdx>,
        bottom: Option<VdSemExprIdx>,
        top_left: Option<VdSemExprIdx>,
        bottom_left: Option<VdSemExprIdx>,
        top_right: Option<VdSemExprIdx>,
        bottom_right: Option<VdSemExprIdx>,
        dispatch: AttachDispatch,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
}

pub type VdSemExprIdx = ArenaIdx<SemExprData>;
pub type VdSemExprIdxRange = ArenaIdxRange<SemExprData>;
pub type VdSemExprArena = Arena<SemExprData>;
pub type VdSemExprArenaRef<'a> = ArenaRef<'a, SemExprData>;
