use husky_coword::Ident;
use husky_hir_ty::HirType;
use husky_task_interface::IsLinkageImpl;
use shifted_unsigned_int::ShiftedU32;
use std::marker::PhantomData;

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
pub enum Instruction<LinkageImpl: IsLinkageImpl> {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        ty: HirType,
        varname: Ident,
        explicit: bool,
    },
    PushLiteralValue {
        value: LinkageImpl::Value,
        ty: HirType,
        explicit: bool,
    },
    WrapInSome {
        number_of_somes: u8,
    },
    CallRoutine {
        resolved_linkage: LinkageImpl,
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
    VirtualStructField {
        field_idx: u8,
        field_binding: Binding,
        field_ty: HirType,
    },
    NewVirtualStruct {
        ty: HirType,
        fields: Vec<Ident>,
    },
    Loop {
        body: InstructionBlockId<LinkageImpl>,
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
        opt_instruction_sheet: Option<InstructionBlockId<LinkageImpl>>,
    },
}

#[derive(Debug)]
pub struct InstructionBlock<LinkageImpl: IsLinkageImpl> {
    instructions: Vec<Instruction<LinkageImpl>>,
}

#[derive(Debug)]
pub struct InstructionStorage<LinkageImpl: IsLinkageImpl> {
    instruction_blocks: Vec<InstructionBlock<LinkageImpl>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstructionBlockId<LinkageImpl: IsLinkageImpl>(ShiftedU32, PhantomData<LinkageImpl>);

impl<LinkageImpl: IsLinkageImpl> InstructionBlockId<LinkageImpl> {
    fn index(self) -> usize {
        self.0.into()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstructionId<LinkageImpl: IsLinkageImpl>(InstructionBlockId<LinkageImpl>, ShiftedU32);

impl<LinkageImpl: IsLinkageImpl> std::ops::Index<InstructionBlockId<LinkageImpl>>
    for InstructionStorage<LinkageImpl>
{
    type Output = InstructionBlock<LinkageImpl>;

    fn index(&self, id: InstructionBlockId<LinkageImpl>) -> &Self::Output {
        &self.instruction_blocks[id.index()]
    }
}
