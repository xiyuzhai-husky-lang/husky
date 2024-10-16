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
pub enum SemMathExprData {
    Notation,
    Binary {
        opr: (),
        dispatch: (),
    },
    Prefix {
        opr: SemMathExprIdx,
        opd: SemMathExprIdx,
        dispatch: (),
    },
    Suffix {
        opd: SemMathExprIdx,
        opr: SemMathExprIdx,
        dispatch: (),
    },
    Attach {
        base: SemMathExprIdx,
        // INVARIANCE: at least one of these are some
        top: Option<SemMathExprIdx>,
        bottom: Option<SemMathExprIdx>,
        top_left: Option<SemMathExprIdx>,
        bottom_left: Option<SemMathExprIdx>,
        top_right: Option<SemMathExprIdx>,
        bottom_right: Option<SemMathExprIdx>,
        dispatch: AttachDispatch,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
}

pub type SemMathExprIdx = ArenaIdx<SemMathExprData>;
