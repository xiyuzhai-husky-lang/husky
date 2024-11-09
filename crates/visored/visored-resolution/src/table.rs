use crate::resolution::{
    command::{VdCompleteCommandResolution, VdCompleteCommandResolutionMap},
    punctuation::VdPunctuationResolution,
};
use latex_command::path::LxCommandPath;
use latex_math_punctuation::{LxMathPunctationMap, LxMathPunctuation};

pub struct VdDefaultResolutionTable {
    punctuation_resolution_map: LxMathPunctationMap<Option<VdPunctuationResolution>>,
    complete_command_resolution_map: VdCompleteCommandResolutionMap,
}

impl VdDefaultResolutionTable {
    pub fn new(
        punctuation_resolution_map: LxMathPunctationMap<Option<VdPunctuationResolution>>,
        command_resolution_map: VdCompleteCommandResolutionMap,
        db: &salsa::Db,
    ) -> Self {
        Self {
            punctuation_resolution_map,
            complete_command_resolution_map: command_resolution_map,
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
