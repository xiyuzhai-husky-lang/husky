use crate::resolution::{
    command::{VdCommandResolution, VdCommandResolutionMap},
    punctuation::VdPunctuationResolution,
};
use latex_command::path::LxCommandPath;
use latex_math_punctuation::{LxMathPunctationMap, LxMathPunctuation};

pub struct VdDefaultResolutionTable {
    punctuation_resolution_map: LxMathPunctationMap<Option<VdPunctuationResolution>>,
    command_resolution_map: VdCommandResolutionMap,
}

impl VdDefaultResolutionTable {
    pub fn new(
        punctuation_resolution_map: LxMathPunctationMap<Option<VdPunctuationResolution>>,
        command_resolution_map: VdCommandResolutionMap,
        db: &salsa::Db,
    ) -> Self {
        Self {
            punctuation_resolution_map,
            command_resolution_map,
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
    type Output = VdCommandResolution;

    fn index(&self, index: LxCommandPath) -> &Self::Output {
        &self.command_resolution_map[&index]
    }
}
