use crate::diag::TypstSourceResult;
use crate::foundations::{elem, TypstContent, TypstContentRefined, TypstStyleChain};
use crate::layout::{
    FixedAlignment, Size, TypstAbsLength, TypstEmLength, TypstFrame, TypstFrameItem, TypstPoint,
};
use crate::math::{
    alignments, scaled_font_size, style_cramped, style_for_subscript, FrameFragment, GlyphFragment,
    MathContext, MathRow, Scaled, TypstAlignmentResult, TypstLayoutMath,
};
use crate::syntax::TypstSynSpan;
use crate::text::TextElem;
use crate::visualize::{TypstFixedStroke, TypstGeometry};

const BRACE_GAP: TypstEmLength = TypstEmLength::new(0.25);
const BRACKET_GAP: TypstEmLength = TypstEmLength::new(0.25);

/// A marker to distinguish under- vs. overlines.
enum LineKind {
    Over,
    Under,
}

/// A horizontal line under content.
///
/// ```example
/// $ underline(1 + 2 + ... + 5) $
/// ```
#[elem(TypstLayoutMath)]
pub struct UnderlineElem {
    /// The content above the line.
    #[required]
    pub body: TypstContent,
}

impl TypstLayoutMath for TypstContentRefined<UnderlineElem> {
    #[husky_typst_macros::time(name = "math.underline", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout_underoverline(ctx, styles, self.body(), self.span(), LineKind::Under)
    }
}

/// A horizontal line over content.
///
/// ```example
/// $ overline(1 + 2 + ... + 5) $
/// ```
#[elem(TypstLayoutMath)]
pub struct OverlineElem {
    /// The content below the line.
    #[required]
    pub body: TypstContent,
}

impl TypstLayoutMath for TypstContentRefined<OverlineElem> {
    #[husky_typst_macros::time(name = "math.overline", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout_underoverline(ctx, styles, self.body(), self.span(), LineKind::Over)
    }
}

/// layout under- or overlined content
fn layout_underoverline(
    ctx: &mut MathContext,
    styles: TypstStyleChain,
    body: &TypstContent,
    span: TypstSynSpan,
    line: LineKind,
) -> TypstSourceResult<()> {
    let (extra_height, content, line_pos, content_pos, baseline, bar_height);
    match line {
        LineKind::Under => {
            let sep = scaled!(ctx, styles, underbar_extra_descender);
            bar_height = scaled!(ctx, styles, underbar_rule_thickness);
            let gap = scaled!(ctx, styles, underbar_vertical_gap);
            extra_height = sep + bar_height + gap;

            content = ctx.layout_fragment(body, styles)?;

            line_pos = TypstPoint::with_y(content.height() + gap + bar_height / 2.0);
            content_pos = TypstPoint::zero();
            baseline = content.ascent()
        }
        LineKind::Over => {
            let sep = scaled!(ctx, styles, overbar_extra_ascender);
            bar_height = scaled!(ctx, styles, overbar_rule_thickness);
            let gap = scaled!(ctx, styles, overbar_vertical_gap);
            extra_height = sep + bar_height + gap;

            let cramped = style_cramped();
            content = ctx.layout_fragment(body, styles.chain(&cramped))?;

            line_pos = TypstPoint::with_y(sep + bar_height / 2.0);
            content_pos = TypstPoint::with_y(extra_height);
            baseline = content.ascent() + extra_height;
        }
    }

    let width = content.width();
    let height = content.height() + extra_height;
    let size = Size::new(width, height);

    let content_class = content.class();
    let mut frame = TypstFrame::soft(size);
    frame.set_baseline(baseline);
    frame.push_frame(content_pos, content.into_frame());
    frame.push(
        line_pos,
        TypstFrameItem::Shape(
            TypstGeometry::Line(TypstPoint::with_x(width)).stroked(TypstFixedStroke {
                paint: TextElem::fill_in(styles).as_decoration(),
                thickness: bar_height,
                ..TypstFixedStroke::default()
            }),
            span,
        ),
    );

    ctx.push(FrameFragment::new(ctx, styles, frame).with_class(content_class));

    Ok(())
}

/// A horizontal brace under content, with an optional annotation below.
///
/// ```example
/// $ underbrace(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(TypstLayoutMath)]
pub struct UnderbraceElem {
    /// The content above the brace.
    #[required]
    pub body: TypstContent,

    /// The optional content below the brace.
    #[positional]
    pub annotation: Option<TypstContent>,
}

impl TypstLayoutMath for TypstContentRefined<UnderbraceElem> {
    #[husky_typst_macros::time(name = "math.underbrace", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout_underoverspreader(
            ctx,
            styles,
            self.body(),
            &self.annotation(styles),
            '⏟',
            BRACE_GAP,
            false,
            self.span(),
        )
    }
}

/// A horizontal brace over content, with an optional annotation above.
///
/// ```example
/// $ overbrace(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(TypstLayoutMath)]
pub struct OverbraceElem {
    /// The content below the brace.
    #[required]
    pub body: TypstContent,

    /// The optional content above the brace.
    #[positional]
    pub annotation: Option<TypstContent>,
}

impl TypstLayoutMath for TypstContentRefined<OverbraceElem> {
    #[husky_typst_macros::time(name = "math.overbrace", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout_underoverspreader(
            ctx,
            styles,
            self.body(),
            &self.annotation(styles),
            '⏞',
            BRACE_GAP,
            true,
            self.span(),
        )
    }
}

/// A horizontal bracket under content, with an optional annotation below.
///
/// ```example
/// $ underbracket(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(TypstLayoutMath)]
pub struct UnderbracketElem {
    /// The content above the bracket.
    #[required]
    pub body: TypstContent,

    /// The optional content below the bracket.
    #[positional]
    pub annotation: Option<TypstContent>,
}

impl TypstLayoutMath for TypstContentRefined<UnderbracketElem> {
    #[husky_typst_macros::time(name = "math.underbrace", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout_underoverspreader(
            ctx,
            styles,
            self.body(),
            &self.annotation(styles),
            '⎵',
            BRACKET_GAP,
            false,
            self.span(),
        )
    }
}

/// A horizontal bracket over content, with an optional annotation above.
///
/// ```example
/// $ overbracket(1 + 2 + ... + 5, "numbers") $
/// ```
#[elem(TypstLayoutMath)]
pub struct OverbracketElem {
    /// The content below the bracket.
    #[required]
    pub body: TypstContent,

    /// The optional content above the bracket.
    #[positional]
    pub annotation: Option<TypstContent>,
}

impl TypstLayoutMath for TypstContentRefined<OverbracketElem> {
    #[husky_typst_macros::time(name = "math.overbracket", span = self.span())]
    fn layout_math(&self, ctx: &mut MathContext, styles: TypstStyleChain) -> TypstSourceResult<()> {
        layout_underoverspreader(
            ctx,
            styles,
            self.body(),
            &self.annotation(styles),
            '⎴',
            BRACKET_GAP,
            true,
            self.span(),
        )
    }
}

/// Layout an over- or underbrace-like object.
#[allow(clippy::too_many_arguments)]
fn layout_underoverspreader(
    ctx: &mut MathContext,
    styles: TypstStyleChain,
    body: &TypstContent,
    annotation: &Option<TypstContent>,
    c: char,
    gap: TypstEmLength,
    reverse: bool,
    span: TypstSynSpan,
) -> TypstSourceResult<()> {
    let font_size = scaled_font_size(ctx, styles);
    let gap = gap.at(font_size);
    let body = ctx.layout_row(body, styles)?;
    let body_class = body.class();
    let body = body.into_fragment(ctx, styles);
    let glyph = GlyphFragment::new(ctx, styles, c, span);
    let stretched = glyph.stretch_horizontal(ctx, body.width(), TypstAbsLength::zero());

    let mut rows = vec![MathRow::new(vec![body]), stretched.into()];

    let (sup_style, sub_style);
    let row_styles = if reverse {
        sup_style = style_for_subscript(styles);
        styles.chain(&sup_style)
    } else {
        sub_style = style_for_subscript(styles);
        styles.chain(&sub_style)
    };

    rows.extend(
        annotation
            .as_ref()
            .map(|annotation| ctx.layout_row(annotation, row_styles))
            .transpose()?,
    );

    let mut baseline = 0;
    if reverse {
        rows.reverse();
        baseline = rows.len() - 1;
    }

    let frame = stack(ctx, styles, rows, FixedAlignment::Center, gap, baseline);
    ctx.push(FrameFragment::new(ctx, styles, frame).with_class(body_class));

    Ok(())
}

/// Stack rows on top of each other.
///
/// Add a `gap` between each row and uses the baseline of the `baseline`th
/// row for the whole frame.
pub(super) fn stack(
    ctx: &MathContext,
    styles: TypstStyleChain,
    rows: Vec<MathRow>,
    align: FixedAlignment,
    gap: TypstAbsLength,
    baseline: usize,
) -> TypstFrame {
    let rows: Vec<_> = rows.into_iter().flat_map(|r| r.rows()).collect();
    let TypstAlignmentResult { points, width } = alignments(&rows);
    let rows: Vec<_> = rows
        .into_iter()
        .map(|row| row.into_aligned_frame(ctx, styles, &points, align))
        .collect();

    let mut y = TypstAbsLength::zero();
    let mut frame = TypstFrame::soft(Size::new(
        width,
        rows.iter().map(|row| row.height()).sum::<TypstAbsLength>()
            + rows.len().saturating_sub(1) as f64 * gap,
    ));

    for (i, row) in rows.into_iter().enumerate() {
        let x = align.position(width - row.width());
        let pos = TypstPoint::new(x, y);
        if i == baseline {
            frame.set_baseline(y + row.baseline());
        }
        y += row.height() + gap;
        frame.push_frame(pos, row);
    }

    frame
}
