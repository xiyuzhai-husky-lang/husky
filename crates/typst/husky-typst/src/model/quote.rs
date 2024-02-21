use crate::diag::TypstSourceResult;
use crate::engine::TypstEngine;
use crate::foundations::{
    cast, elem, IsTypstElem, Label, Show, ShowSet, Smart, TypstContent, TypstContentRefined,
    TypstStyleChain, TypstStyles,
};
use crate::layout::{
    BlockElem, HElem, Spacing, TypstAlignment, TypstEmLength, TypstPadElem, VElem,
};
use crate::model::{CitationForm, CiteTypstElem};
use crate::text::{SmartQuoteElem, SpaceElem, TextElem};

/// Displays a quote alongside an optional attribution.
///
/// # Example
/// ```example
/// Plato is often misquoted as the author of #quote[I know that I know
/// nothing], however, this is a derivation form his original quote:
///
/// #set quote(block: true)
///
/// #quote(attribution: [Plato])[
///   ... ἔοικα γοῦν τούτου γε σμικρῷ τινι αὐτῷ τούτῳ σοφώτερος εἶναι, ὅτι
///   ἃ μὴ οἶδα οὐδὲ οἴομαι εἰδέναι.
/// ]
/// #quote(attribution: [from the Henry Cary literal translation of 1897])[
///   ... I seem, then, in just this little thing to be wiser than this man at
///   any rate, that what I do not know I do not think I know either.
/// ]
/// ```
///
/// By default block quotes are padded left and right by `{1em}`, alignment and
/// padding can be controlled with show rules:
/// ```example
/// #set quote(block: true)
/// #show quote: set align(center)
/// #show quote: set pad(x: 5em)
///
/// #quote[
///   You cannot pass... I am a servant of the Secret Fire, wielder of the
///   flame of Anor. You cannot pass. The dark fire will not avail you,
///   flame of Udûn. Go back to the Shadow! You cannot pass.
/// ]
/// ```
#[elem(ShowSet, Show)]
pub struct QuoteElem {
    /// Whether this is a block quote.
    ///
    /// ```example
    /// An inline citation would look like
    /// this: #quote(
    ///   attribution: [René Descartes]
    /// )[
    ///   cogito, ergo sum
    /// ], and a block equation like this:
    /// #quote(
    ///   block: true,
    ///   attribution: [JFK]
    /// )[
    ///   Ich bin ein Berliner.
    /// ]
    /// ```
    block: bool,

    /// Whether double quotes should be added around this quote.
    ///
    /// The double quotes used are inferred from the `quotes` property on
    /// [smartquote]($smartquote), which is affected by the `lang` property on
    /// [text]($text).
    ///
    /// - `{true}`: Wrap this quote in double quotes.
    /// - `{false}`: Do not wrap this quote in double quotes.
    /// - `{auto}`: Infer whether to wrap this quote in double quotes based on
    ///   the `block` property. If `block` is `{false}`, double quotes are
    ///   automatically added.
    ///
    /// ```example
    /// #set text(lang: "de")
    ///
    /// Ein deutsch-sprechender Author
    /// zitiert unter umständen JFK:
    /// #quote[Ich bin ein Berliner.]
    ///
    /// #set text(lang: "en")
    ///
    /// And an english speaking one may
    /// translate the quote:
    /// #quote[I am a Berliner.]
    /// ```
    quotes: Smart<bool>,

    /// The attribution of this quote, usually the author or source. Can be a
    /// label pointing to a bibliography entry or any content. By default only
    /// displayed for block quotes, but can be changed using a `{show}` rule.
    ///
    /// ```example
    /// #quote(attribution: [René Descartes])[
    ///   cogito, ergo sum
    /// ]
    ///
    /// #show quote.where(block: false): it => {
    ///   ["] + h(0pt, weak: true) + it.body + h(0pt, weak: true) + ["]
    ///   if it.attribution != none [ (#it.attribution)]
    /// }
    ///
    /// #quote(
    ///   attribution: link("https://typst.app/home")[typst.com]
    /// )[
    ///   Compose papers faster
    /// ]
    ///
    /// #set quote(block: true)
    ///
    /// #quote(attribution: <tolkien54>)[
    ///   You cannot pass... I am a servant
    ///   of the Secret Fire, wielder of the
    ///   flame of Anor. You cannot pass. The
    ///   dark fire will not avail you, flame
    ///   of Udûn. Go back to the Shadow! You
    ///   cannot pass.
    /// ]
    ///
    /// #bibliography("works.bib", style: "apa")
    /// ```
    #[borrowed]
    attribution: Option<Attribution>,

    /// The quote.
    #[required]
    body: TypstContent,
}

/// Attribution for a [quote](QuoteElem).
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Attribution {
    Content(TypstContent),
    Label(Label),
}

cast! {
    Attribution,
    self => match self {
        Self::Content(content) => content.into_value(),
        Self::Label(label) => label.into_value(),
    },
    content: TypstContent => Self::Content(content),
    label: Label => Self::Label(label),
}

impl Show for TypstContentRefined<QuoteElem> {
    #[husky_typst_macros::time(name = "quote", span = self.span())]
    fn show(
        &self,
        _: &mut TypstEngine,
        styles: TypstStyleChain,
    ) -> TypstSourceResult<TypstContent> {
        let mut realized = self.body().clone();
        let block = self.block(styles);

        if self.quotes(styles) == Smart::Custom(true) || !block {
            // Add zero-width weak spacing to make the quotes "sticky".
            let hole = HElem::hole().pack();
            let quote = SmartQuoteElem::new().with_double(true).pack();
            realized = TypstContent::sequence([quote.clone(), hole.clone(), realized, hole, quote]);
        }

        if block {
            realized = BlockElem::new()
                .with_body(Some(realized))
                .pack()
                .spanned(self.span());

            if let Some(attribution) = self.attribution(styles).as_ref() {
                let mut seq = vec![TextElem::packed('—'), SpaceElem::new().pack()];

                match attribution {
                    Attribution::Content(content) => {
                        seq.push(content.clone());
                    }
                    Attribution::Label(label) => {
                        seq.push(
                            CiteTypstElem::new(*label)
                                .with_form(Some(CitationForm::Prose))
                                .pack()
                                .spanned(self.span()),
                        );
                    }
                }

                // Use v(0.9em, weak: true) bring the attribution closer to the
                // quote.
                let weak_v = VElem::weak(Spacing::Rel(TypstEmLength::new(0.9).into())).pack();
                realized += weak_v + TypstContent::sequence(seq).aligned(TypstAlignment::END);
            }

            realized = TypstPadElem::new(realized).pack();
        } else if let Some(Attribution::Label(label)) = self.attribution(styles) {
            realized +=
                SpaceElem::new().pack() + CiteTypstElem::new(*label).pack().spanned(self.span());
        }

        Ok(realized)
    }
}

impl ShowSet for TypstContentRefined<QuoteElem> {
    fn show_set(&self, _: TypstStyleChain) -> TypstStyles {
        let x = TypstEmLength::new(1.0).into();
        let above = TypstEmLength::new(2.4).into();
        let below = TypstEmLength::new(1.8).into();
        let mut out = TypstStyles::new();
        out.set(TypstPadElem::set_left(x));
        out.set(TypstPadElem::set_right(x));
        out.set(BlockElem::set_above(VElem::block_around(above)));
        out.set(BlockElem::set_below(VElem::block_around(below)));
        out
    }
}
