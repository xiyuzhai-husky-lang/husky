use unicode_math_class::MathClass;

use crate::diag::TypstSourceResult;
use crate::foundations::{elem, TypstContent, TypstContentRefined, TypstStyleChain};
use crate::math::{EquationTypstElem, Limits, MathContext, TypstLayoutMath};

/// Forced use of a certain math class.
///
/// This is useful to treat certain symbols as if they were of a different
/// class, e.g. to make a symbol behave like a relation. The class of a symbol
/// defines the way it is laid out, including spacing around it, and how its
/// scripts are attached by default. Note that the latter can always be
/// overridden using [`{limits}`](math.limits) and [`{scripts}`](math.scripts).
///
/// # Example
/// ```example
/// #let loves = math.class(
///   "relation",
///   sym.suit.heart,
/// )
///
/// $x loves y and y loves 5$
/// ```
#[elem(TypstLayoutMath)]
pub struct ClassTypstElem {
    /// The class to apply to the content.
    #[required]
    pub class: MathClass,

    /// The content to which the class is applied.
    #[required]
    pub body: TypstContent,
}

impl TypstLayoutMath for TypstContentRefined<ClassTypstElem> {
    #[husky_typst_macros::time(name = "math.class", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        let class = *self.class();
        let style = EquationTypstElem::set_class(Some(class)).wrap();
        let mut fragment = ctx.layout_fragment(self.body(), styles.chain(&style))?;
        fragment.set_class(class);
        fragment.set_limits(Limits::for_class(class));
        ctx.push(fragment);
        Ok(())
    }
}
