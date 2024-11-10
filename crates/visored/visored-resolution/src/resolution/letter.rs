use latex_math_letter::letter::LxMathLetter;
use rustc_hash::FxHashMap;
use visored_item_path::path::VdItemPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdLetterResolution {
    Item(VdItemPath),
}

pub type VdLetterResolutionMap = FxHashMap<LxMathLetter, VdLetterResolution>;

impl VdLetterResolution {
    pub const NATURAL_NUMBER: Self = Self::Item(VdItemPath::NATURAL_NUMBER);
    pub const RATIONAL_NUMBER: Self = Self::Item(VdItemPath::RATIONAL_NUMBER);
    pub const INTEGER: Self = Self::Item(VdItemPath::INTEGER);
    pub const REAL_NUMBER: Self = Self::Item(VdItemPath::REAL_NUMBER);
    pub const COMPLEX_NUMBER: Self = Self::Item(VdItemPath::COMPLEX_NUMBER);
}
