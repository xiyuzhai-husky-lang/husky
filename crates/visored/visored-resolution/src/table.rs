use crate::resolution::punctuation::VdPunctuationResolution;
use latex_math_punctuation::LxMathPunctuation;

pub struct VdDefaultResolutionTable {}

impl VdDefaultResolutionTable {
    pub fn new(db: &salsa::Db) -> Self {
        Self {}
    }

    pub fn new_standard(db: &salsa::Db) -> Self {
        Self::new(db)
    }

    pub fn resolve_punctuation(&self, punctuation: LxMathPunctuation) -> VdPunctuationResolution {
        let _ = punctuation;
        todo!()
    }
}
