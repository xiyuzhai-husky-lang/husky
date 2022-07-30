mod condition_flow;
mod id;
mod opn;
mod pattern_match;
mod sheet;

pub use condition_flow::*;
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use husky_print_utils::p;
use husky_vm_primitive_value::PrimitiveValueData;
pub use id::{InstructionId, InstructionSource};
pub use opn::*;
pub use pattern_match::*;
pub use sheet::InstructionSheet;

use crate::*;
use avec::Avec;
use husky_entity_route::EntityRoutePtr;
use husky_file::FilePtr;
use husky_text::TextRange;
use husky_word::{CustomIdentifier, IdentPairDict, Identifier};
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
        T: Deref<Target = S> + std::fmt::Debug + Send + Sync + RefUnwindSafe,
    > InstructionSource for T
{
    fn instruction_id(&self) -> InstructionId {
        S::instruction_id(self)
    }
    fn file(&self) -> FilePtr {
        S::file(self)
    }

    fn text_range(&self) -> TextRange {
        S::text_range(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionVariant {
    PushVariable {
        stack_idx: VMStackIdx,
        binding: Binding,
        range: TextRange,
        ty: EntityRoutePtr,
        varname: Identifier,
        explicit: bool,
    },
    PushPrimitiveValue {
        value: PrimitiveValueData,
        ty: EntityRoutePtr,
        explicit: bool,
    },
    PushEnumKindLiteral(EnumKindValue),
    FieldAccessInterpreted {
        field_idx: u8,
        field_binding: Binding,
        field_ty: EntityRoutePtr,
    },
    CallRoutine {
        linkage_fp: __LinkageFp,
        nargs: u8,
        output_ty: EntityRoutePtr,
    },
    CallInterpreted {
        routine_uid: EntityUid,
        nargs: u8,
        has_this: bool,
        output_ty: EntityRoutePtr,
    },
    NewVirtualStruct {
        ty: EntityRoutePtr,
        fields: Vec<CustomIdentifier>,
    },
    Loop {
        body: Arc<InstructionSheet>,
        loop_kind: VMLoopKind,
    },
    Return {
        output_ty: EntityRoutePtr,
    },
    BreakIfFalse,
    Break,
    Assert,
    ConditionFlow {
        branches: Avec<VMConditionBranch>,
    },
    PatternMatch {
        branches: Avec<VMPatternBranch>,
    },
    EntityFeature {
        feature_uid: EntityUid,
        ty: EntityRoutePtr,
    },
    PushEntityFp {
        opt_linkage: Option<__Linkage>,
        ty: EntityRoutePtr,
        opt_instruction_sheet: Option<Arc<InstructionSheet>>,
    },
}
