use std::fmt::{self, Debug, Formatter};
use std::ops::Range;

use ecow::EcoString;

use crate::layout::{TypstAbsLength, TypstEmLength};
use crate::syntax::TypstSynSpan;
use crate::text::{Lang, TypstFont};
use crate::visualize::{TypstFixedStroke, TypstPaint};

/// A run of shaped text.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TypstTextItem {
    /// The font the glyphs are contained in.
    pub font: TypstFont,
    /// The font size.
    pub size: TypstAbsLength,
    /// Glyph color.
    pub fill: TypstPaint,
    /// Glyph stroke.
    pub stroke: Option<TypstFixedStroke>,
    /// The natural language of the text.
    pub lang: Lang,
    /// The item's plain text.
    pub text: EcoString,
    /// The glyphs. The number of glyphs may be different from the number of
    /// characters in the plain text due to e.g. ligatures.
    pub glyphs: Vec<TypstGlyph>,
}

impl TypstTextItem {
    /// The width of the text run.
    pub fn width(&self) -> TypstAbsLength {
        self.glyphs
            .iter()
            .map(|g| g.x_advance)
            .sum::<TypstEmLength>()
            .at(self.size)
    }
}

impl Debug for TypstTextItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Text(")?;
        self.text.fmt(f)?;
        f.write_str(")")
    }
}

/// A glyph in a run of shaped text.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypstGlyph {
    /// The glyph's index in the font.
    pub id: u16,
    /// The advance width of the glyph.
    pub x_advance: TypstEmLength,
    /// The horizontal offset of the glyph.
    pub x_offset: TypstEmLength,
    /// The range of the glyph in its item's text. The range's length may
    /// be more than one due to multi-byte UTF-8 encoding or ligatures.
    pub range: Range<u16>,
    /// The source code location of the text.
    pub span: (TypstSynSpan, u16),
}

impl TypstGlyph {
    /// The range of the glyph in its item's text.
    pub fn range(&self) -> Range<usize> {
        usize::from(self.range.start)..usize::from(self.range.end)
    }
}
