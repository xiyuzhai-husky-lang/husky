use crate::{
    decompose::{DcnArena, DcnArenaRef, DcnIdx, DcnIdxRange, Decompose},
    parser::VdCnlParser,
};

pub enum VdCnlExprData {}

pub type VdCnlExprIdx = DcnIdx<VdCnlExprData>;
pub type VdCnlExprIdxRange = DcnIdxRange<VdCnlExprData>;
pub type VdCnlExprArena = DcnArena<VdCnlExprData>;
pub type VdCnlExprArenaRef<'a> = DcnArenaRef<'a, VdCnlExprData>;

impl Decompose for VdCnlExprData {
    fn arena_mut<'a>(parser: &'a mut VdCnlParser) -> &'a mut DcnArena<Self> {
        parser.expr_arena_mut()
    }
}
