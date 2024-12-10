use crate::{
    decompose::{DcnArena, DcnArenaRef, DcnIdx, DcnIdxRange, Decompose},
    parser::VdCnlParser,
};

pub enum VdCnlSentenceData {}

pub type VdCnlSentenceIdx = DcnIdx<VdCnlSentenceData>;
pub type VdCnlSentenceIdxRange = DcnIdxRange<VdCnlSentenceData>;
pub type VdCnlSentenceArena = DcnArena<VdCnlSentenceData>;
pub type VdCnlSentenceArenaRef<'a> = DcnArenaRef<'a, VdCnlSentenceData>;

impl Decompose for VdCnlSentenceData {
    fn arena_mut<'a>(parser: &'a mut VdCnlParser) -> &'a mut DcnArena<Self> {
        parser.sentence_arena_mut()
    }
}
