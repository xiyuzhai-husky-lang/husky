use crate::{
    decompose::{DcnArena, DcnArenaRef, DcnIdx, DcnIdxRange, Decompose},
    parser::VdCnlParser,
};

pub enum VdCnlPhraseData {}

pub type VdCnlPhraseIdx = DcnIdx<VdCnlPhraseData>;
pub type VdCnlPhraseIdxRange = DcnIdxRange<VdCnlPhraseData>;
pub type VdCnlPhraseArena = DcnArena<VdCnlPhraseData>;
pub type VdCnlPhraseArenaRef<'a> = DcnArenaRef<'a, VdCnlPhraseData>;

impl Decompose for VdCnlPhraseData {
    fn arena_mut<'a>(parser: &'a mut VdCnlParser) -> &'a mut DcnArena<Self> {
        parser.phrase_arena_mut()
    }
}
