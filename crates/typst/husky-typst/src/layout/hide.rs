use smallvec::smallvec;

use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{elem, Show, TypstContent, TypstContentRefined, TypstStyleChain};
use crate::introspection::{MetaTypstElem, TypstMeta};

/// Hides content without affecting layout.
///
/// The `hide` function allows you to hide content while the layout still 'sees'
/// it. This is useful to create whitespace that is exactly as large as some
/// content. It may also be useful to redact content because its arguments are
/// not included in the output.
///
/// # Example
/// ```example
/// Hello Jane \
/// #hide[Hello] Joe
/// ```
#[elem(Show)]
pub struct HideElem {
    /// The content to hide.
    #[required]
    pub body: TypstContent,
}

impl Show for TypstContentRefined<HideElem> {
    #[husky_typst_macros::time(name = "hide", span = self.span())]
    fn show(&self, _: &mut TypstEngine, _: TypstStyleChain) -> TypstSourceResult<TypstContent> {
        Ok(self
            .body()
            .clone()
            .styled(MetaTypstElem::set_data(smallvec![TypstMeta::Hide])))
    }
}
