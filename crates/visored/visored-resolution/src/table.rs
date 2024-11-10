use crate::resolution::{
    command::{VdCompleteCommandResolution, VdCompleteCommandResolutionMap},
    letter::VdLetterResolutionMap,
    punctuation::{VdPunctuationResolution, VdPunctuationResolutionMap},
};
use latex_command::path::LxCommandPath;
use latex_math_punctuation::{LxMathPunctuation, LxMathPunctuationMap};

pub struct VdDefaultResolutionTable {
    punctuation_resolution_map: VdPunctuationResolutionMap,
    complete_command_resolution_map: VdCompleteCommandResolutionMap,
    letter_resolution_map: VdLetterResolutionMap,
}

impl VdDefaultResolutionTable {
    pub fn new(
        punctuation_resolution_map: VdPunctuationResolutionMap,
        complete_command_resolution_map: VdCompleteCommandResolutionMap,
        letter_resolution_map: VdLetterResolutionMap,
    ) -> Self {
        Self {
            punctuation_resolution_map,
            complete_command_resolution_map,
            letter_resolution_map,
        }
    }
}

impl std::ops::Index<LxMathPunctuation> for VdDefaultResolutionTable {
    type Output = Option<VdPunctuationResolution>;

    fn index(&self, index: LxMathPunctuation) -> &Self::Output {
        &self.punctuation_resolution_map[index]
    }
}

impl std::ops::Index<LxCommandPath> for VdDefaultResolutionTable {
    type Output = VdCompleteCommandResolution;

    fn index(&self, index: LxCommandPath) -> &Self::Output {
        &self.complete_command_resolution_map[&index]
    }
}
