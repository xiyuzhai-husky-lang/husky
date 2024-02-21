use ecow::EcoString;

use crate::diag::{bail, SourceResult, StrResult};
use crate::engine::TypstEngine;
use crate::foundations::{
    cast, elem, Args, Array, Construct, Datetime, Smart, StyleChain, TypstContent,
    TypstContentRefined, TypstValue,
};
use crate::introspection::{Introspector, ManualPageCounter};
use crate::layout::{LayoutRoot, Page, PageElem};

/// The root element of a document and its metadata.
///
/// All documents are automatically wrapped in a `document` element. You cannot
/// create a document element yourself. This function is only used with
/// [set rules]($styling/#set-rules) to specify document metadata. Such a set
/// rule must appear before any of the document's contents.
///
/// ```example
/// #set document(title: [Hello])
///
/// This has no visible output, but
/// embeds metadata into the PDF!
/// ```
///
/// Note that metadata set with this function is not rendered within the
/// document. Instead, it is embedded in the compiled PDF file.
#[elem(Construct, LayoutRoot)]
pub struct DocumentElem {
    /// The document's title. This is often rendered as the title of the
    /// PDF viewer window.
    ///
    /// While this can be arbitrary content, PDF viewers only support plain text
    /// titles, so the conversion might be lossy.
    #[ghost]
    pub title: Option<TypstContent>,

    /// The document's authors.
    #[ghost]
    pub author: Author,

    /// The document's keywords.
    #[ghost]
    pub keywords: Keywords,

    /// The document's creation date.
    ///
    /// If this is `{auto}` (default), Typst uses the current date and time.
    /// Setting it to `{none}` prevents Typst from embedding any creation date
    /// into the PDF metadata.
    ///
    /// The year component must be at least zero in order to be embedded into a
    /// PDF.
    #[ghost]
    pub date: Smart<Option<Datetime>>,

    /// The page runs.
    #[internal]
    #[variadic]
    pub children: Vec<TypstContent>,
}

impl Construct for DocumentElem {
    fn construct(_: &mut TypstEngine, args: &mut Args) -> SourceResult<TypstContent> {
        bail!(args.span, "can only be used in set rules")
    }
}

impl LayoutRoot for TypstContentRefined<DocumentElem> {
    #[husky_typst_macros::time(name = "document", span = self.span())]
    fn layout_root(
        &self,
        engine: &mut TypstEngine,
        styles: StyleChain,
    ) -> SourceResult<TypstDocument> {
        let mut pages = Vec::with_capacity(self.children().len());
        let mut page_counter = ManualPageCounter::new();

        let children = self.children();
        let mut iter = children.iter().peekable();

        while let Some(mut child) = iter.next() {
            let outer_styles = styles;
            let mut styles = styles;
            if let Some((elem, local_styles)) = child.to_styled() {
                styles = outer_styles.chain(local_styles);
                child = elem;
            }

            if let Some(page) = child.to_packed::<PageElem>() {
                let extend_to = iter.peek().and_then(|&next| {
                    *next
                        .to_styled()
                        .map_or(next, |(elem, _)| elem)
                        .to_packed::<PageElem>()?
                        .clear_to()?
                });
                let run = page.layout(engine, styles, &mut page_counter, extend_to)?;
                pages.extend(run);
            } else {
                bail!(child.span(), "unexpected document child");
            }
        }

        Ok(TypstDocument {
            pages,
            title: DocumentElem::title_in(styles).map(|content| content.plain_text()),
            author: DocumentElem::author_in(styles).0,
            keywords: DocumentElem::keywords_in(styles).0,
            date: DocumentElem::date_in(styles),
            introspector: Introspector::default(),
        })
    }
}

/// A list of authors.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Author(Vec<EcoString>);

cast! {
    Author,
    self => self.0.into_value(),
    v: EcoString => Self(vec![v]),
    v: Array => Self(v.into_iter().map(TypstValue::cast).collect::<StrResult<_>>()?),
}

/// A list of keywords.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Keywords(Vec<EcoString>);

cast! {
    Keywords,
    self => self.0.into_value(),
    v: EcoString => Self(vec![v]),
    v: Array => Self(v.into_iter().map(TypstValue::cast).collect::<StrResult<_>>()?),
}

/// A finished document with metadata and page frames.
#[derive(Debug, Default, Clone)]
pub struct TypstDocument {
    /// The document's finished pages.
    pub pages: Vec<Page>,
    /// The document's title.
    pub title: Option<EcoString>,
    /// The document's author.
    pub author: Vec<EcoString>,
    /// The document's keywords.
    pub keywords: Vec<EcoString>,
    /// The document's creation date.
    pub date: Smart<Option<Datetime>>,
    /// Provides the ability to execute queries on the document.
    pub introspector: Introspector,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_is_send_and_sync() {
        fn ensure_send_and_sync<T: Send + Sync>() {}
        ensure_send_and_sync::<TypstDocument>();
    }
}
