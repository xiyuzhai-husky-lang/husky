use crate::resolution::{
    command::{VdCompleteCommandResolution, VdCompleteCommandResolutionMap},
    letter::{VdLetterResolution, VdLetterResolutionMap},
    punctuation::{VdPunctuationResolution, VdPunctuationResolutionMap},
};
use latex_command::path::LxCommandPath;
use latex_math_letter::letter::LxMathLetter;
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

impl VdDefaultResolutionTable {
    pub fn resolve_punctuation(
        &self,
        punctuation: LxMathPunctuation,
    ) -> Option<VdPunctuationResolution> {
        self.punctuation_resolution_map[punctuation]
    }

    pub fn resolve_complete_command(
        &self,
        command_path: LxCommandPath,
    ) -> Option<&VdCompleteCommandResolution> {
        self.complete_command_resolution_map.get(&command_path)
    }

    pub fn resolve_letter(&self, letter: LxMathLetter) -> Option<VdLetterResolution> {
        self.letter_resolution_map.get(&letter).copied()
    }
}
