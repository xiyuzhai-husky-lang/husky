use super::*;
use latex_command::path::LxCommandPath;
use latex_math_letter::LxMathLetter;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdCommandResolution {
    Letter(LxMathLetter),
    Todo,
}

pub type VdCommandResolutionMap = FxHashMap<LxCommandPath, VdCommandResolution>;

impl VdCommandResolution {
    pub const ALPHA: Self = Self::Letter(LxMathLetter::LowerAlpha);
    pub const BETA: Self = Self::Letter(LxMathLetter::LowerBeta);
    pub const GAMMA: Self = Self::Letter(LxMathLetter::LowerGamma);
    pub const PI: Self = Self::Letter(LxMathLetter::LowerPi);
}
