use unicode_math_class::MathClass;

use crate::diag::TypstSourceResult;
use crate::foundations::{
    elem, func, IsTypstElem, Resolve, Smart, TypstContent, TypstContentRefined, TypstStyleChain,
};
use crate::layout::{Length, Rel, TypstAbsLength, TypstEmLength};
use crate::math::{
    GlyphFragment, MathContext, MathFragment, Scaled, SpacingFragment, TypstLayoutMath,
};
use crate::text::TextElem;

/// How much less high scaled delimiters can be than what they wrap.
pub(super) const DELIM_SHORT_FALL: TypstEmLength = TypstEmLength::new(0.1);

/// Scales delimiters.
///
/// While matched delimiters scale by default, this can be used to scale
/// unmatched delimiters and to control the delimiter scaling more precisely.
#[elem(title = "Left/Right", TypstLayoutMath)]
pub struct LrElem {
    /// The size of the brackets, relative to the height of the wrapped content.
    pub size: Smart<Rel<Length>>,

    /// The delimited content, including the delimiters.
    #[required]
    #[parse(
        let mut body = TypstContent::empty();
        for (i, arg) in args.all::<TypstContent>()?.into_iter().enumerate() {
            if i > 0 {
                body += TextElem::packed(',');
            }
            body += arg;
        }
        body
    )]
    pub body: TypstContent,
}

impl TypstLayoutMath for TypstContentRefined<LrElem> {
    #[husky_typst_macros::time(name = "math.lr", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        let mut body = self.body();
        if let Some(elem) = body.to_packed::<LrElem>() {
            if elem.size(styles).is_auto() {
                body = elem.body();
            }
        }

        let mut fragments = ctx.layout_fragments(body, styles)?;
        let axis = scaled!(ctx, styles, axis_height);
        let max_extent = fragments
            .iter()
            .map(|fragment| (fragment.ascent() - axis).max(fragment.descent() + axis))
            .max()
            .unwrap_or_default();

        let height = self
            .size(styles)
            .unwrap_or(Rel::one())
            .resolve(styles)
            .relative_to(2.0 * max_extent);

        // Scale up fragments at both ends.
        match fragments.as_mut_slice() {
            [one] => scale(ctx, styles, one, height, None),
            [first, .., last] => {
                scale(ctx, styles, first, height, Some(MathClass::Opening));
                scale(ctx, styles, last, height, Some(MathClass::Closing));
            }
            _ => {}
        }

        // Handle MathFragment::Variant fragments that should be scaled up.
        for fragment in &mut fragments {
            if let MathFragment::Variant(ref mut variant) = fragment {
                if variant.mid_stretched == Some(false) {
                    variant.mid_stretched = Some(true);
                    scale(ctx, styles, fragment, height, Some(MathClass::Large));
                }
            }
        }

        // Remove weak SpacingFragment immediately after the opening or immediately
        // before the closing.
        let original_len = fragments.len();
        let mut index = 0;
        fragments.retain(|fragment| {
            index += 1;
            (index != 2 && index + 1 != original_len)
                || !matches!(
                    fragment,
                    MathFragment::Spacing(SpacingFragment { weak: true, .. })
                )
        });

        ctx.extend(fragments);

        Ok(())
    }
}

/// Scales delimiters vertically to the nearest surrounding `{lr()}` group.
///
/// ```example
/// $ { x mid(|) sum_(i=1)^n w_i|f_i (x)| < 1 } $
/// ```
#[elem(TypstLayoutMath)]
pub struct MidElem {
    /// The content to be scaled.
    #[required]
    pub body: TypstContent,
}

impl TypstLayoutMath for TypstContentRefined<MidElem> {
    #[husky_typst_macros::time(name = "math.mid", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        let mut fragments = ctx.layout_fragments(self.body(), styles)?;

        for fragment in &mut fragments {
            match fragment {
                MathFragment::Glyph(glyph) => {
                    let mut new = glyph.clone().into_variant();
                    new.mid_stretched = Some(false);
                    *fragment = MathFragment::Variant(new);
                }
                MathFragment::Variant(variant) => {
                    variant.mid_stretched = Some(false);
                }
                _ => {}
            }
        }

        ctx.extend(fragments);
        Ok(())
    }
}

/// Scale a math fragment to a height.
fn scale(
    ctx: &mut MathContext,
    styles: TypstStyleChain,
    fragment: &mut MathFragment,
    height: TypstAbsLength,
    apply: Option<MathClass>,
) {
    if matches!(
        fragment.class(),
        MathClass::Opening | MathClass::Closing | MathClass::Fence
    ) {
        let glyph = match fragment {
            MathFragment::Glyph(glyph) => glyph.clone(),
            MathFragment::Variant(variant) => {
                GlyphFragment::new(ctx, styles, variant.c, variant.span)
            }
            _ => return,
        };

        let short_fall = DELIM_SHORT_FALL.at(glyph.font_size);
        let mut stretched = glyph.stretch_vertical(ctx, height, short_fall);
        stretched.center_on_axis(ctx);

        *fragment = MathFragment::Variant(stretched);
        if let Some(class) = apply {
            fragment.set_class(class);
        }
    }
}

/// Floors an expression.
///
/// ```example
/// $ floor(x/2) $
/// ```
#[func]
pub fn floor(
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Smart<Rel<Length>>>,
    /// The expression to floor.
    body: TypstContent,
) -> TypstContent {
    delimited(body, '⌊', '⌋', size)
}

/// Ceils an expression.
///
/// ```example
/// $ ceil(x/2) $
/// ```
#[func]
pub fn ceil(
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Smart<Rel<Length>>>,
    /// The expression to ceil.
    body: TypstContent,
) -> TypstContent {
    delimited(body, '⌈', '⌉', size)
}

/// Rounds an expression.
///
/// ```example
/// $ round(x/2) $
/// ```
#[func]
pub fn round(
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Smart<Rel<Length>>>,
    /// The expression to round.
    body: TypstContent,
) -> TypstContent {
    delimited(body, '⌊', '⌉', size)
}

/// Takes the absolute value of an expression.
///
/// ```example
/// $ abs(x/2) $
/// ```
#[func]
pub fn abs(
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Smart<Rel<Length>>>,
    /// The expression to take the absolute value of.
    body: TypstContent,
) -> TypstContent {
    delimited(body, '|', '|', size)
}

/// Takes the norm of an expression.
///
/// ```example
/// $ norm(x/2) $
/// ```
#[func]
pub fn norm(
    /// The size of the brackets, relative to the height of the wrapped content.
    #[named]
    size: Option<Smart<Rel<Length>>>,
    /// The expression to take the norm of.
    body: TypstContent,
) -> TypstContent {
    delimited(body, '‖', '‖', size)
}

fn delimited(
    body: TypstContent,
    left: char,
    right: char,
    size: Option<Smart<Rel<Length>>>,
) -> TypstContent {
    let span = body.span();
    let mut elem = LrElem::new(TypstContent::sequence([
        TextElem::packed(left),
        body,
        TextElem::packed(right),
    ]));
    // Push size only if size is provided
    if let Some(size) = size {
        elem.push_size(size);
    }
    elem.pack().spanned(span)
}
