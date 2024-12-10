use crate::{
    decompose::{DcnArena, DcnArenaRef, DcnIdx, DcnIdxRange, Decompose},
    parser::VdCnlParser,
};

pub enum VdCnlClauseData {}

pub type VdCnlClauseIdx = DcnIdx<VdCnlClauseData>;
pub type VdCnlClauseIdxRange = DcnIdxRange<VdCnlClauseData>;
pub type VdCnlClauseArena = DcnArena<VdCnlClauseData>;
pub type VdCnlClauseArenaRef<'a> = DcnArenaRef<'a, VdCnlClauseData>;

impl Decompose for VdCnlClauseData {
    fn arena_mut<'a>(parser: &'a mut VdCnlParser) -> &'a mut DcnArena<Self> {
        parser.clause_arena_mut()
    }
}
