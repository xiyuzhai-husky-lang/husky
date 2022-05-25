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
use word::{CustomIdentifier, IdentPairDict, Identifier};

#[derive(Debug)]
pub struct Instruction {
    pub variant: InstructionVariant,
    pub src: Arc<dyn InstructionSource>,
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.variant == other.variant && self.src.instruction_id() == other.src.instruction_id()
    }
}

impl Eq for Instruction {}

impl Instruction {
    pub fn new(kind: InstructionVariant, src: Arc<dyn InstructionSource>) -> Self {
        Self { variant: kind, src }
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
pub enum InstructionVariant {
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
        field_access_contract: EagerContract,
    },
    CallCompiled {
        linkage: Linkage,
    },
    CallInterpreted {
        routine_uid: EntityUid,
        nargs: u8,
        has_this: bool,
    },
    NewVirtualStruct {
        fields: IdentPairDict<FieldLiason>,
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
    NewXmlFromTag {
        tag_kind: XmlTagKind,
        props: Vec<CustomIdentifier>,
        n_child_expr: u8,
    },
    NewXmlFromValue {
        ty: EntityRoutePtr,
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
    BinaryAssign(Option<PureBinaryOpr>),
    Prefix(PrefixOpr),
    Cast(CastOpn),
    Incr,
    Decr,
}

pub fn binary_assign<'stack, 'eval>(
    opt_binary_opr: Option<PureBinaryOpr>,
    lopd: &mut StackValue<'stack, 'eval>,
    ropd: StackValue<'stack, 'eval>,
) -> VMRuntimeResult<()> {
    match lopd {
        StackValue::RefMut { ref mut value, .. } => {
            value.assign(if let Some(binary_opr) = opt_binary_opr {
                let lopd_value = value.take_copyable();
                binary_opr
                    .act_on_primitives(lopd_value, ropd.take_copyable())?
                    .into()
            } else {
                ropd
            });
        }
        _ => panic!(),
    }
    Ok(())
}

pub fn incr<'stack, 'eval>(opd: &mut StackValue<'stack, 'eval>) {
    let opd_primitive = opd.take_copyable();
    match opd {
        StackValue::RefMut { value, owner, gen } => {
            value.assign(StackValue::Copyable(match opd_primitive {
                CopyableValue::I32(i) => (i + 1).into(),
                CopyableValue::F32(_) => todo!(),
                CopyableValue::B32(_) => todo!(),
                CopyableValue::B64(_) => todo!(),
                CopyableValue::Bool(_) => todo!(),
                CopyableValue::Void(_) => todo!(),
                CopyableValue::EnumKind(_) => todo!(),
            }))
        }

        _ => panic!(),
    }
}

pub fn decr<'stack, 'eval>(opd: &mut StackValue<'stack, 'eval>) {
    let opd_primitive = opd.take_copyable();
    match opd {
        StackValue::RefMut { value, owner, gen } => {
            value.assign(StackValue::Copyable(match opd_primitive {
                CopyableValue::I32(i) => (i - 1).into(),
                CopyableValue::F32(_) => todo!(),
                CopyableValue::B32(_) => todo!(),
                CopyableValue::B64(_) => todo!(),
                CopyableValue::Bool(_) => todo!(),
                CopyableValue::Void(_) => todo!(),
                CopyableValue::EnumKind(_) => todo!(),
            }))
        }

        _ => panic!(),
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
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as i32,
        },
        StackValue::Owned(_) => todo!(),
        StackValue::GlobalPure(_) => todo!(),
        StackValue::GlobalRef(_) => todo!(),
        StackValue::LocalRef(value) => todo!(),
        StackValue::RefMut { value, owner, gen } => todo!(),
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
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as u32,
        },
        StackValue::Owned(_) => todo!(),
        StackValue::GlobalPure(_) => todo!(),
        StackValue::GlobalRef(_) => todo!(),
        StackValue::LocalRef(value) => todo!(),
        StackValue::RefMut { value, owner, gen } => todo!(),
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
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => todo!(),
        },
        StackValue::Owned(_) => todo!(),
        StackValue::GlobalPure(_) => todo!(),
        StackValue::GlobalRef(_) => todo!(),
        StackValue::LocalRef(value) => todo!(),
        StackValue::RefMut { value, owner, gen } => todo!(),
    }
}
