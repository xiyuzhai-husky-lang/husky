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
        opr: VisoredSemMathExprIdx,
        opd: VisoredSemMathExprIdx,
        dispatch: (),
    },
    Suffix {
        opd: VisoredSemMathExprIdx,
        opr: VisoredSemMathExprIdx,
        dispatch: (),
    },
    Attach {
        base: VisoredSemMathExprIdx,
        // INVARIANCE: at least one of these are some
        top: Option<VisoredSemMathExprIdx>,
        bottom: Option<VisoredSemMathExprIdx>,
        top_left: Option<VisoredSemMathExprIdx>,
        bottom_left: Option<VisoredSemMathExprIdx>,
        top_right: Option<VisoredSemMathExprIdx>,
        bottom_right: Option<VisoredSemMathExprIdx>,
        dispatch: AttachDispatch,
    },
    UniadicChain,
    VariadicChain,
    UniadicArray,
    VariadicArray,
}

pub type VisoredSemMathExprIdx = ArenaIdx<SemMathExprData>;
