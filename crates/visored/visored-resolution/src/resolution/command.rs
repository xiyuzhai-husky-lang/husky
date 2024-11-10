use super::*;
use latex_command::path::LxCommandPath;
use latex_math_letter::LxMathLetter;
use rustc_hash::FxHashMap;
use visored_item_path::VdItemPath;
use visored_opr::opr::VdBaseOpr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdCompleteCommandResolution {
    Letter(LxMathLetter),
    Item(VdItemPath),
    Opr(VdBaseOpr),
    Frac,
    Sqrt,
    Text,
    Todo,
}

pub type VdCompleteCommandResolutionMap = FxHashMap<LxCommandPath, VdCompleteCommandResolution>;

impl VdCompleteCommandResolution {
    pub const INT: Self = Self::Opr(VdBaseOpr::INTEGRAL);
    pub const SUM: Self = Self::Opr(VdBaseOpr::SUM);
    pub const PROD: Self = Self::Opr(VdBaseOpr::PROD);
    pub const TIMES: Self = Self::Opr(VdBaseOpr::TIMES);
    pub const OTIMES: Self = Self::Opr(VdBaseOpr::OTIMES);
    pub const LOWER_ALPHA: Self = Self::Letter(LxMathLetter::LOWER_ALPHA);
    pub const LOWER_BETA: Self = Self::Letter(LxMathLetter::LOWER_BETA);
    pub const LOWER_GAMMA: Self = Self::Letter(LxMathLetter::LOWER_GAMMA);
    pub const LOWER_PI: Self = Self::Letter(LxMathLetter::LOWER_PI);
}
