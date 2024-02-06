use husky_coword::Ident;
use husky_hir_ty::HirType;
use husky_linkage::linkage::Linkage;
use husky_task_interface::value::LiteralValue;
use idx_arena::Arena;
use idx_arena::{ArenaIdx, ArenaIdxRange};

// ad hoc
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VMStackIdx;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct Binding;

// ad hoc
#[derive(Debug, PartialEq, Eq)]
pub struct VMLoopKind;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        ty: HirType,
        varname: Ident,
        explicit: bool,
    },
    PushLiteralValue {
        value: LiteralValue,
        ty: HirType,
        explicit: bool,
    },
    CallRoutine {
        resolved_linkage: Linkage,
        nargs: u8,
        return_ty: HirType,
        discard: bool,
    },
    CallInterpreted {
        // routine_uid: EntityUid,
        nargs: u8,
        return_ty: HirType,
        discard: bool,
    },
    Loop {
        body: InstructionIdxRange,
        loop_kind: VMLoopKind,
    },
    Return {
        return_ty: HirType,
    },
    BreakIfFalse,
    Break,
    Assert,
    Require,
    ConditionFlow {
        // branches: Avec<VMConditionBranch>,
    },
    PatternMatch {
        // branches: Avec<VMPatternBranch>,
    },
    EntityFeature {
        // feature_uid: EntityUid,
        ty: HirType,
    },
    PushEntityFp {
        // opt_linkage: Option<__LinkageGroup>,
        ty: HirType,
        opt_instruction_sheet: Option<InstructionIdxRange>,
    },
}

pub type InstructionArena = Arena<Instruction>;
pub type InstructionIdx = ArenaIdx<Instruction>;
pub type InstructionIdxRange = ArenaIdxRange<Instruction>;
