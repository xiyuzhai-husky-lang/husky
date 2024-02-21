mod context;
mod error;
mod fragment;
mod row;

use self::context::TypstMathContext;
use self::error::TypstEquationResult;

/// Layout for math elements.
pub(crate) trait TypstLayoutMath {
    /// Layout the element, producing fragment in the context.
    fn layout_math(
        &self,
        ctx: &mut TypstMathContext,
        styles: TypstStyleChain,
    ) -> TypstEquationResult<()>;
}

type TypstStyleChain = ();
