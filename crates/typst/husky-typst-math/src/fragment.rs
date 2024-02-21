#[enum_class::from_variants]
#[derive(Debug, Clone)]
pub enum MathFragment {
    Glyph(GlyphFragment),
    Variant(VariantFragment),
    Frame(FrameFragment),
    Spacing(SpacingFragment),
    Space(TypstAbsLength),
    Linebreak,
    Align,
}

#[derive(Debug, Clone)]
pub struct GlyphFragment;

#[derive(Debug, Clone)]
pub struct VariantFragment;

#[derive(Debug, Clone)]
pub struct FrameFragment;

#[derive(Debug, Clone)]
pub struct SpacingFragment;

#[derive(Debug, Clone)]
pub struct TypstAbsLength;
