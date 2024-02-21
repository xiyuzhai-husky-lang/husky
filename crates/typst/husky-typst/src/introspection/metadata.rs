use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{
    elem, Behave, Behaviour, Show, TypstContent, TypstContentRefined, TypstStyleChain, TypstValue,
};
use crate::introspection::TypstLocatable;

/// Exposes a value to the query system without producing visible content.
///
/// This element can be retrieved with the [`query`]($query) function and from
/// the command with [`typst query`]($reference/meta/query/#cli-queries). Its
/// purpose is to expose an arbitrary value to the introspection system. To
/// identify a metadata value among others, you can attach a [`label`]($label)
/// to it and query for that label.
///
/// The `metadata` element is especially useful for command line queries because
/// it allows you to expose arbitrary values to the outside world.
///
/// ```example
/// // Put metadata somewhere.
/// #metadata("This is a note") <note>
///
/// // And find it from anywhere else.
/// #locate(loc => {
///   query(<note>, loc).first().value
/// })
/// ```
#[elem(Behave, Show, TypstLocatable)]
pub struct MetadataElem {
    /// The value to embed into the document.
    #[required]
    pub value: TypstValue,
}

impl Show for TypstContentRefined<MetadataElem> {
    fn show(
        &self,
        _: &mut TypstEngine,
        _styles: TypstStyleChain,
    ) -> TypstSourceResult<TypstContent> {
        Ok(TypstContent::empty())
    }
}

impl Behave for TypstContentRefined<MetadataElem> {
    fn behaviour(&self) -> Behaviour {
        Behaviour::Invisible
    }
}
