mod condition_flow;
mod id;
mod opn;
mod pattern_match;
mod sheet;

pub use condition_flow::*;
pub use id::{InstructionId, InstructionSource};
pub use opn::*;
pub use pattern_match::*;
pub use sheet::InstructionSheet;

use crate::*;
use avec::Avec;
use husky_entity_route::Ty;
use husky_text::{FileRanged, TextRange};
use husky_word::{CustomIdentifier, Identifier};
use std::{ops::Deref, panic::RefUnwindSafe, sync::Arc};

#[derive(Debug)]
pub struct Instruction {
    pub variant: InstructionVariant,
    pub src: Arc<dyn InstructionSource>,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.variant == other.variant && self.ins_id() == other.ins_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(variant: InstructionVariant, src: Arc<dyn InstructionSource>) -> Self {
        Self { variant, src }
    }

    pub fn ins_id(&self) -> InstructionId {
        self.src.instruction_id()
    }
}

impl<
        S: InstructionSource,
        T: Deref<Target = S> + std::fmt::Debug + Send + Sync + RefUnwindSafe + FileRanged,
    > InstructionSource for T
{
    fn instruction_id(&self) -> InstructionId {
        S::instruction_id(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionVariant {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        range: TextRange,
        ty: Ty,
        varname: Identifier,
        explicit: bool,
    },
    PushLiteralValue {
        value: __Register<'static>,
        ty: Ty,
        explicit: bool,
    },
    WrapInSome {
        number_of_somes: u8,
    },
    CallRoutine {
        resolved_linkage: __ResolvedLinkage,
        nargs: u8,
        output_ty: Ty,
        discard: bool,
    },
    CallInterpreted {
        routine_uid: EntityUid,
        nargs: u8,
        output_ty: Ty,
        discard: bool,
    },
    VirtualStructField {
        field_idx: u8,
        field_binding: Binding,
        field_ty: Ty,
    },
    NewVirtualStruct {
        ty: Ty,
        fields: Vec<CustomIdentifier>,
    },
    Loop {
        body: Arc<InstructionSheet>,
        loop_kind: VMLoopKind,
    },
    Return {
        output_ty: Ty,
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
        ty: Ty,
    },
    PushEntityFp {
        opt_linkage: Option<__Linkage>,
        ty: Ty,
        opt_instruction_sheet: Option<Arc<InstructionSheet>>,
    },
}
