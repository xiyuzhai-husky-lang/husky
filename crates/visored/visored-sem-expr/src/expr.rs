mod attach;
mod notation;
mod prefix;
mod suffix;
mod uniadic_array;
mod uniadic_chain;
mod variadic_array;
mod variadic_chain;

use attach::AttachDispatch;
use idx_arena::ArenaIdx;

/// It's a tree of both form and meaning
#[derive(Debug, PartialEq, Eq)]
pub enum SemExprData {
    Notation,
    Binary {
        opr: (),
        dispatch: (),
    },
    Prefix {
        opr: VisoredSemExprIdx,
        opd: VisoredSemExprIdx,
        dispatch: (),
    },
    Suffix {
        opd: VisoredSemExprIdx,
        opr: VisoredSemExprIdx,
        dispatch: (),
    },
    Attach {
        base: VisoredSemExprIdx,
        // INVARIANCE: at least one of these are some
        top: Option<VisoredSemExprIdx>,
        bottom: Option<VisoredSemExprIdx>,
        top_left: Option<VisoredSemExprIdx>,
        bottom_left: Option<VisoredSemExprIdx>,
        top_right: Option<VisoredSemExprIdx>,
        bottom_right: Option<VisoredSemExprIdx>,
        dispatch: AttachDispatch,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
}

pub type VisoredSemExprIdx = ArenaIdx<SemExprData>;
