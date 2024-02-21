use std::num::NonZeroUsize;
use std::str::FromStr;

use crate::diag::{bail, At, StrResult, TypstSourceResult};
use crate::engine::TypstEngine;
use crate::foundations::{
    cast, elem, scope, IsTypstElem, Label, Show, Smart, TypstContent, TypstContentRefined,
    TypstShowSet, TypstStyleChain, TypstStyles,
};
use crate::introspection::{Counter, CounterUpdate, Location, TypstCount, TypstLocatable};
use crate::layout::{HElem, Ratio, TypstAbsLength, TypstEmLength, TypstLength};
use crate::model::{Numbering, NumberingPattern, ParagraphTypstElem, TypstDestination};
use crate::text::{SuperElem, TextElem, TextSize};
use crate::util::NonZeroExt;
use crate::visualize::{LineElem, TypstStroke};

/// A footnote.
///
/// Includes additional remarks and references on the same page with footnotes.
/// A footnote will insert a superscript number that links to the note at the
/// bottom of the page. Notes are numbered sequentially throughout your document
/// and can break across multiple pages.
///
/// To customize the appearance of the entry in the footnote listing, see
/// [`footnote.entry`]($footnote.entry). The footnote itself is realized as a
/// normal superscript, so you can use a set rule on the [`super`]($super)
/// function to customize it. You can also apply a show rule to customize
/// only the footnote marker (superscript number) in the running text.
///
/// # Example
/// ```example
/// Check the docs for more details.
/// #footnote[https://typst.app/docs]
/// ```
///
/// The footnote automatically attaches itself to the preceding word, even if
/// there is a space before it in the markup. To force space, you can use the
/// string `[#" "]` or explicit [horizontal spacing]($h).
///
/// By giving a label to a footnote, you can have multiple references to it.
///
/// ```example
/// You can edit Typst documents online.
/// #footnote[https://typst.app/app] <fn>
/// Checkout Typst's website. @fn
/// And the online app. #footnote(<fn>)
/// ```
///
/// _Note:_ Set and show rules in the scope where `footnote` is called may not
/// apply to the footnote's content. See [here][issue] for more information.
///
/// [issue]: https://github.com/typst/typst/issues/1467#issuecomment-1588799440
#[elem(scope, TypstLocatable, Show, TypstCount)]
pub struct FootnoteTypstElem {
    /// How to number footnotes.
    ///
    /// By default, the footnote numbering continues throughout your document.
    /// If you prefer per-page footnote numbering, you can reset the footnote
    /// [counter]($counter) in the page [header]($page.header). In the future,
    /// there might be a simpler way to achieve this.
    ///
    /// ```example
    /// #set footnote(numbering: "*")
    ///
    /// Footnotes:
    /// #footnote[Star],
    /// #footnote[Dagger]
    /// ```
    #[borrowed]
    #[default(Numbering::Pattern(NumberingPattern::from_str("1").unwrap()))]
    pub numbering: Numbering,

    /// The content to put into the footnote. Can also be the label of another
    /// footnote this one should point to.
    #[required]
    pub body: FootnoteBody,
}

#[scope]
impl FootnoteTypstElem {
    #[elem]
    type FootnoteEntry;
}

impl FootnoteTypstElem {
    /// Creates a new footnote that the passed content as its body.
    pub fn with_content(content: TypstContent) -> Self {
        Self::new(FootnoteBody::Content(content))
    }

    /// Creates a new footnote referencing the footnote with the specified label.
    pub fn with_label(label: Label) -> Self {
        Self::new(FootnoteBody::Reference(label))
    }

    /// Tests if this footnote is a reference to another footnote.
    pub fn is_ref(&self) -> bool {
        matches!(self.body(), FootnoteBody::Reference(_))
    }

    /// Returns the content of the body of this footnote if it is not a ref.
    pub fn body_content(&self) -> Option<&TypstContent> {
        match self.body() {
            FootnoteBody::Content(content) => Some(content),
            _ => None,
        }
    }
}

impl TypstContentRefined<FootnoteTypstElem> {
    /// Returns the location of the definition of this footnote.
    pub fn declaration_location(&self, engine: &TypstEngine) -> StrResult<Location> {
        match self.body() {
            FootnoteBody::Reference(label) => {
                let element = engine.introspector.query_label(*label)?;
                let footnote = element
                    .to_packed::<FootnoteTypstElem>()
                    .ok_or("referenced element should be a footnote")?;
                footnote.declaration_location(engine)
            }
            _ => Ok(self.location().unwrap()),
        }
    }
}

impl Show for TypstContentRefined<FootnoteTypstElem> {
    #[husky_typst_macros::time(name = "footnote", span = self.span())]
    fn show(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
    ) -> TypstSourceResult<TypstContent> {
        let loc = self.declaration_location(engine).at(self.span())?;
        let numbering = self.numbering(styles);
        let counter = Counter::of(FootnoteTypstElem::elem());
        let num = counter.at(engine, loc)?.display(engine, numbering)?;
        let sup = SuperElem::new(num).pack().spanned(self.span());
        let loc = loc.variant(1);
        // Add zero-width weak spacing to make the footnote "sticky".
        Ok(HElem::hole().pack() + sup.linked(TypstDestination::Location(loc)))
    }
}

impl TypstCount for TypstContentRefined<FootnoteTypstElem> {
    fn update(&self) -> Option<CounterUpdate> {
        (!self.is_ref()).then(|| CounterUpdate::Step(NonZeroUsize::ONE))
    }
}

/// The body of a footnote can be either some content or a label referencing
/// another footnote.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum FootnoteBody {
    Content(TypstContent),
    Reference(Label),
}

cast! {
    FootnoteBody,
    self => match self {
        Self::Content(v) => v.into_value(),
        Self::Reference(v) => v.into_value(),
    },
    v: TypstContent => Self::Content(v),
    v: Label => Self::Reference(v),
}

/// An entry in a footnote list.
///
/// This function is not intended to be called directly. Instead, it is used
/// in set and show rules to customize footnote listings.
///
/// _Note:_ Set and show rules for `footnote.entry` must be defined at the
/// beginning of the document in order to work correctly.
/// See [here](https://github.com/typst/typst/issues/1348#issuecomment-1566316463)
/// for more information.
///
/// ```example
/// #show footnote.entry: set text(red)
///
/// My footnote listing
/// #footnote[It's down here]
/// has red text!
/// ```
#[elem(name = "entry", title = "Footnote Entry", Show, TypstShowSet)]
pub struct FootnoteEntry {
    /// The footnote for this entry. It's location can be used to determine
    /// the footnote counter state.
    ///
    /// ```example
    /// #show footnote.entry: it => {
    ///   let loc = it.note.location()
    ///   numbering(
    ///     "1: ",
    ///     ..counter(footnote).at(loc),
    ///   )
    ///   it.note.body
    /// }
    ///
    /// Customized #footnote[Hello]
    /// listing #footnote[World! 🌏]
    /// ```
    #[required]
    pub note: TypstContentRefined<FootnoteTypstElem>,

    /// The separator between the document body and the footnote listing.
    ///
    /// ```example
    /// #set footnote.entry(
    ///   separator: repeat[.]
    /// )
    ///
    /// Testing a different separator.
    /// #footnote[
    ///   Unconventional, but maybe
    ///   not that bad?
    /// ]
    /// ```
    #[default(
        LineElem::new()
            .with_length(Ratio::new(0.3).into())
            .with_stroke(TypstStroke {
                thickness: Smart::Custom(TypstAbsLength::pt(0.5).into()),
                ..Default::default()
            })
            .pack()
    )]
    pub separator: TypstContent,

    /// The amount of clearance between the document body and the separator.
    ///
    /// ```example
    /// #set footnote.entry(clearance: 3em)
    ///
    /// Footnotes also need ...
    /// #footnote[
    ///   ... some space to breathe.
    /// ]
    /// ```
    #[default(TypstEmLength::new(1.0).into())]
    #[resolve]
    pub clearance: TypstLength,

    /// The gap between footnote entries.
    ///
    /// ```example
    /// #set footnote.entry(gap: 0.8em)
    ///
    /// Footnotes:
    /// #footnote[Spaced],
    /// #footnote[Apart]
    /// ```
    #[default(TypstEmLength::new(0.5).into())]
    #[resolve]
    pub gap: TypstLength,

    /// The indent of each footnote entry.
    ///
    /// ```example
    /// #set footnote.entry(indent: 0em)
    ///
    /// Footnotes:
    /// #footnote[No],
    /// #footnote[Indent]
    /// ```
    #[default(TypstEmLength::new(1.0).into())]
    pub indent: TypstLength,
}

impl Show for TypstContentRefined<FootnoteEntry> {
    #[husky_typst_macros::time(name = "footnote.entry", span = self.span())]
    fn show(
        &self,
        engine: &mut TypstEngine,
        styles: TypstStyleChain,
    ) -> TypstSourceResult<TypstContent> {
        let note = self.note();
        let number_gap = TypstEmLength::new(0.05);
        let default = TypstStyleChain::default();
        let numbering = note.numbering(default);
        let counter = Counter::of(FootnoteTypstElem::elem());
        let Some(loc) = note.location() else {
            bail!(
                self.span(), "footnote entry must have a location";
                hint: "try using a query or a show rule to customize the footnote instead"
            );
        };

        let num = counter.at(engine, loc)?.display(engine, numbering)?;
        let sup = SuperElem::new(num)
            .pack()
            .spanned(self.span())
            .linked(TypstDestination::Location(loc))
            .backlinked(loc.variant(1));
        Ok(TypstContent::sequence([
            HElem::new(self.indent(styles).into()).pack(),
            sup,
            HElem::new(number_gap.into()).with_weak(true).pack(),
            note.body_content().unwrap().clone(),
        ]))
    }
}

impl TypstShowSet for TypstContentRefined<FootnoteEntry> {
    fn show_set(&self, _: TypstStyleChain) -> TypstStyles {
        let text_size = TypstEmLength::new(0.85);
        let leading = TypstEmLength::new(0.5);
        let mut out = TypstStyles::new();
        out.set(ParagraphTypstElem::set_leading(leading.into()));
        out.set(TextElem::set_size(TextSize(text_size.into())));
        out
    }
}

cast! {
    FootnoteTypstElem,
    v: TypstContent => v.unpack::<Self>().unwrap_or_else(Self::with_content)
}
