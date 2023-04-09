#[derive(Debug, Default, PartialEq, Eq)]
pub struct HoleRegistry {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HoleIdn {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HoleKind {
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
}
