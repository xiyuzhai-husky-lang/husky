use super::*;
use visored_prelude::division::VdDivisionLevel;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirBlockMeta {
    Paragraph,
    Sentence,
    Division(VdDivisionLevel),
}
