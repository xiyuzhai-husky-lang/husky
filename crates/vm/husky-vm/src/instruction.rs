mod condition_flow;
mod id;
mod opn;
mod pattern_match;
mod sheet;

pub use condition_flow::*;
pub use id::{InstructionId, InstructionSource};
pub use opn::*;
pub use pattern_match::*;
pub use sheet::Instructions;

use crate::*;
use avec::Avec;
use husky_coword::Ident;
use husky_text::{HasSourceRange, TextRange};
use std::{ops::Deref, panic::RefUnwindSafe, sync::Arc};

#[derive(Debug)]
pub struct Instruction {
    pub data: InstructionData,
    pub src: Arc<dyn InstructionSource>,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.ins_id() == other.ins_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(data: InstructionData, src: Arc<dyn InstructionSource>) -> Self {
        Self { data, src }
    }

    pub fn ins_id(&self) -> InstructionId {
        self.src.instruction_id()
    }
}

impl<
        S: InstructionSource,
        T: Deref<Target = S> + std::fmt::Debug + Send + Sync + RefUnwindSafe + HasSourceRange,
    > InstructionSource for T
{
    fn instruction_id(&self) -> InstructionId {
        S::instruction_id(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionData {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        range: TextRange,
        ty: EtherealTerm,
        varname: Ident,
        explicit: bool,
    },
    PushLiteralValue {
        // value: PrimitiveValue,
        ty: EtherealTerm,
        explicit: bool,
    },
    WrapInSome {
        number_of_somes: u8,
    },
    CallRoutine {
        resolved_linkage: __ResolvedLinkage,
        nargs: u8,
        return_ty: EtherealTerm,
        discard: bool,
    },
    CallInterpreted {
        routine_uid: EntityUid,
        nargs: u8,
        return_ty: EtherealTerm,
        discard: bool,
    },
    VirtualStructField {
        field_idx: u8,
        field_binding: Binding,
        field_ty: EtherealTerm,
    },
    NewVirtualStruct {
        ty: EtherealTerm,
        fields: Vec<Ident>,
    },
    Loop {
        body: Instructions,
        loop_kind: VMLoopKind,
    },
    Return {
        return_ty: EtherealTerm,
    },
    BreakIfFalse,
    Break,
    Assert,
    Require,
    ConditionFlow {
        branches: Avec<VMConditionBranch>,
    },
    PatternMatch {
        branches: Avec<VMPatternBranch>,
    },
    EntityFeature {
        feature_uid: EntityUid,
        ty: EtherealTerm,
    },
    PushEntityFp {
        opt_linkage: Option<__LinkageGroup>,
        ty: EtherealTerm,
        opt_instruction_sheet: Option<Instructions>,
    },
}
