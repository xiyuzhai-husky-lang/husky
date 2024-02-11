use crate::diag::SourceResult;
use crate::engine::TexEngine;
use crate::foundations::{
    elem, func, Func, IsTexElem, Show, StyleChain, TexContent, TexContentRefined,
};
use crate::introspection::Locatable;
use crate::syntax::Span;

/// Provides access to the location of content.
///
/// This is useful in combination with [queries]($query), [counters]($counter),
/// [state]($state), and [links]($link). See their documentation for more
/// details.
///
/// ```example
/// #locate(loc => [
///   My location: \
///   #loc.position()!
/// ])
/// ```
#[func]
pub fn locate(
    /// The span of the `locate` call.
    span: Span,
    /// A function that receives a [`location`]($location). Its return value is
    /// displayed in the document.
    ///
    /// This function is called once for each time the content returned by
    /// `locate` appears in the document. That makes it possible to generate
    /// content that depends on its own location in the document.
    func: Func,
) -> TexContent {
    LocateElem::new(func).pack().spanned(span)
}

/// Executes a `locate` call.
#[elem(Locatable, Show)]
struct LocateElem {
    /// The function to call with the location.
    #[required]
    func: Func,
}

impl Show for TexContentRefined<LocateElem> {
    #[husky_tex_macros::time(name = "locate", span = self.span())]
    fn show(&self, engine: &mut TexEngine, _: StyleChain) -> SourceResult<TexContent> {
        let location = self.location().unwrap();
        Ok(self.func().call(engine, [location])?.display())
    }
}
