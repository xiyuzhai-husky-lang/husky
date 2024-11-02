use crate::resolution::punctuation::VdPunctuationResolution;
use latex_math_punctuation::{LxMathPunctationMap, LxMathPunctuation};

pub struct VdDefaultResolutionTable {
    punctuation_resolution_map: LxMathPunctationMap<Option<VdPunctuationResolution>>,
}

impl VdDefaultResolutionTable {
    pub fn new(
        punctuation_resolution_map: LxMathPunctationMap<Option<VdPunctuationResolution>>,
        db: &salsa::Db,
    ) -> Self {
        Self {
            punctuation_resolution_map,
        }
    }
}

impl std::ops::Index<LxMathPunctuation> for VdDefaultResolutionTable {
    type Output = Option<VdPunctuationResolution>;

    fn index(&self, index: LxMathPunctuation) -> &Self::Output {
        &self.punctuation_resolution_map[index]
    }
}
