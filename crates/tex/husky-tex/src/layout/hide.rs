use smallvec::smallvec;

use crate::diag::SourceResult;
use crate::engine::TexEngine;
use crate::foundations::{elem, Show, StyleChain, TexContent, TexContentRefined};
use crate::introspection::{Meta, MetaTexElem};

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
    pub body: TexContent,
}

impl Show for TexContentRefined<HideElem> {
    #[husky_tex_macros::time(name = "hide", span = self.span())]
    fn show(&self, _: &mut TexEngine, _: StyleChain) -> SourceResult<TexContent> {
        Ok(self
            .body()
            .clone()
            .styled(MetaTexElem::set_data(smallvec![Meta::Hide])))
    }
}
