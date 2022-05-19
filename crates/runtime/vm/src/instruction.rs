mod condition_flow;
mod id;
mod opr;
mod pattern_match;
mod sheet;

pub use condition_flow::*;
pub use id::{InstructionId, InstructionSource};
pub use opr::*;
pub use pattern_match::*;
use print_utils::p;
pub use sheet::InstructionSheet;

use crate::*;
use avec::Avec;
use entity_route::EntityRoutePtr;
use file::FilePtr;
use std::{ops::Deref, panic::RefUnwindSafe, sync::Arc};
use text::TextRange;
use word::Identifier;

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub src: Arc<dyn InstructionSource>,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.src.instruction_id() == other.src.instruction_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(kind: InstructionKind, src: Arc<dyn InstructionSource>) -> Self {
        Self { kind, src }
    }

    pub fn id(&self) -> InstructionId {
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
pub enum InstructionKind {
    PushVariable {
        stack_idx: StackIdx,
        binding: Binding,
        range: TextRange,
        ty: EntityRoutePtr,
        varname: Identifier,
    },
    PushPrimitiveLiteral(CopyableValue),
    PushEnumKindLiteral(EnumKindValue),
    FieldAccessCompiled {
        linkage: Linkage,
    },
    FieldAccessInterpreted {
        field_idx: u8,
        contract: EagerContract,
    },
    RoutineCallCompiled {
        linkage: Linkage,
    },
    RoutineCallInterpreted {
        routine: EntityUid,
        nargs: u8,
    },
    NewVirtualStruct {
        fields: Vec<FieldContract>,
    },
    OprOpn {
        opn: OprOpn,
        this_ty: EntityRoutePtr,
        this_range: TextRange,
    },
    Loop {
        body: Arc<InstructionSheet>,
        loop_kind: VMLoopKind,
    },
    Return,
    BreakIfFalse,
    Break,
    Assert,
    ConditionFlow {
        branches: Avec<VMConditionBranch>,
    },
    PatternMatch {
        branches: Avec<VMPatternBranch>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InitKind {
    Let,
    Var,
    Decl,
}

impl std::fmt::Display for InitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            InitKind::Let => "let",
            InitKind::Var => "var",
            InitKind::Decl => "decl",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OprOpn {
    PureBinary(PureBinaryOpr),
    Assign(AssignOpn),
    Prefix(PrefixOpr),
    Cast(CastOpn),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AssignOpn {
    Binary(Option<PureBinaryOpr>),
    Incr,
    Decr,
}

impl AssignOpn {
    pub fn act_on<'stack, 'eval>(
        self,
        lopd: &mut StackValue<'stack, 'eval>,
        ropd: StackValue<'stack, 'eval>,
    ) -> VMRuntimeResult<()> {
        match self {
            AssignOpn::Binary(opt_binary_opr) => match lopd {
                StackValue::LocalRefMut { ref mut value, .. } => {
                    value.assign(if let Some(binary_opr) = opt_binary_opr {
                        let lopd_value = value.primitive();
                        binary_opr
                            .act_on_primitives(lopd_value, ropd.primitive())?
                            .into()
                    } else {
                        ropd
                    });
                }
                _ => panic!(),
            },
            AssignOpn::Incr => todo!(),
            AssignOpn::Decr => todo!(),
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CastOpn {
    AsI32,
    AsB32,
    AsF32,
}

impl CastOpn {
    pub fn act_on<'stack, 'eval>(
        self,
        opd: StackValue<'stack, 'eval>,
    ) -> StackValue<'stack, 'eval> {
        match self {
            CastOpn::AsI32 => StackValue::Copyable(cast_as_i32(opd).into()),
            CastOpn::AsB32 => StackValue::Copyable(cast_as_b32(opd).into()),
            CastOpn::AsF32 => StackValue::Copyable(cast_as_f32(opd).into()),
        }
    }
}

fn cast_as_i32<'stack, 'eval>(opd: StackValue<'stack, 'eval>) -> i32 {
    match opd {
        StackValue::Moved => todo!(),
        StackValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i,
            CopyableValue::F32(_) => todo!(),
            CopyableValue::B32(b) => b as i32,
            CopyableValue::B64(_) => todo!(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as i32,
        },
        StackValue::Owned(_) => todo!(),
        StackValue::GlobalPure(_) => todo!(),
        StackValue::GlobalRef(_) => todo!(),
        StackValue::LocalRef { value, owner, gen } => todo!(),
        StackValue::LocalRefMut { value, owner, gen } => todo!(),
    }
}

fn cast_as_b32<'stack, 'eval>(opd: StackValue<'stack, 'eval>) -> u32 {
    match opd {
        StackValue::Moved => todo!(),
        StackValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i as u32,
            CopyableValue::F32(_) => todo!(),
            CopyableValue::B32(_) => todo!(),
            CopyableValue::B64(_) => todo!(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as u32,
        },
        StackValue::Owned(_) => todo!(),
        StackValue::GlobalPure(_) => todo!(),
        StackValue::GlobalRef(_) => todo!(),
        StackValue::LocalRef { value, owner, gen } => todo!(),
        StackValue::LocalRefMut { value, owner, gen } => todo!(),
    }
}

fn cast_as_f32<'stack, 'eval>(opd: StackValue<'stack, 'eval>) -> f32 {
    match opd {
        StackValue::Moved => todo!(),
        StackValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i as f32,
            CopyableValue::F32(_) => todo!(),
            CopyableValue::B32(b) => todo!(),
            CopyableValue::B64(_) => todo!(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void => todo!(),
            CopyableValue::EnumKind(enum_kind) => todo!(),
        },
        StackValue::Owned(_) => todo!(),
        StackValue::GlobalPure(_) => todo!(),
        StackValue::GlobalRef(_) => todo!(),
        StackValue::LocalRef { value, owner, gen } => todo!(),
        StackValue::LocalRefMut { value, owner, gen } => todo!(),
    }
}
