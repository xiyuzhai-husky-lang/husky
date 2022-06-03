mod condition_flow;
mod id;
mod opn;
mod pattern_match;
mod sheet;

pub use condition_flow::*;
pub use id::{InstructionId, InstructionSource};
pub use opn::*;
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
    pub fn new(variant: InstructionVariant, src: Arc<dyn InstructionSource>) -> Self {
        Self { variant, src }
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
        stack_idx: VMStackIdx,
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
        field_binding: Binding,
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
        fields: IdentPairDict<MemberLiason>,
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

pub fn binary_assign<'vm, 'eval>(
    opt_binary_opr: Option<PureBinaryOpr>,
    lopd: &mut TempValue<'vm, 'eval>,
    ropd: TempValue<'vm, 'eval>,
) -> VMRuntimeResult<()> {
    match lopd {
        TempValue::CopyableOrFullyOwnedMut { ref mut value, .. } => {
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

pub fn incr<'vm, 'eval>(opd: &mut TempValue<'vm, 'eval>) {
    let opd_primitive = opd.take_copyable();
    match opd {
        TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => {
            value.assign(TempValue::Copyable(match opd_primitive {
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

pub fn decr<'vm, 'eval>(opd: &mut TempValue<'vm, 'eval>) {
    let opd_primitive = opd.take_copyable();
    match opd {
        TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => {
            value.assign(TempValue::Copyable(match opd_primitive {
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
    pub fn act_on<'vm, 'eval>(self, opd: TempValue<'vm, 'eval>) -> TempValue<'vm, 'eval> {
        match self {
            CastOpn::AsI32 => TempValue::Copyable(cast_as_i32(opd).into()),
            CastOpn::AsB32 => TempValue::Copyable(cast_as_b32(opd).into()),
            CastOpn::AsF32 => TempValue::Copyable(cast_as_f32(opd).into()),
        }
    }
}

fn cast_as_i32<'vm, 'eval>(opd: TempValue<'vm, 'eval>) -> i32 {
    match opd {
        TempValue::Moved => todo!(),
        TempValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i,
            CopyableValue::F32(_) => todo!(),
            CopyableValue::B32(b) => b as i32,
            CopyableValue::B64(_) => todo!(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as i32,
        },
        TempValue::EvalOwned(_) => todo!(),
        TempValue::EvalPure(_) => todo!(),
        TempValue::EvalRef(_) => todo!(),
        TempValue::FullyOwnedRef(value) => todo!(),
        TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
        TempValue::TempOwned(_) => todo!(),
        TempValue::PartiallyOwnedRef(_) => todo!(),
        TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
    }
}

fn cast_as_b32<'vm, 'eval>(opd: TempValue<'vm, 'eval>) -> u32 {
    match opd {
        TempValue::Moved => todo!(),
        TempValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i as u32,
            CopyableValue::F32(_) => todo!(),
            CopyableValue::B32(_) => todo!(),
            CopyableValue::B64(_) => todo!(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as u32,
        },
        TempValue::EvalOwned(_) => todo!(),
        TempValue::EvalPure(_) => todo!(),
        TempValue::EvalRef(_) => todo!(),
        TempValue::FullyOwnedRef(value) => todo!(),
        TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
        TempValue::TempOwned(_) => todo!(),
        TempValue::PartiallyOwnedRef(_) => todo!(),
        TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
    }
}

fn cast_as_f32<'vm, 'eval>(opd: TempValue<'vm, 'eval>) -> f32 {
    match opd {
        TempValue::Moved => todo!(),
        TempValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i as f32,
            CopyableValue::F32(_) => todo!(),
            CopyableValue::B32(b) => todo!(),
            CopyableValue::B64(_) => todo!(),
            CopyableValue::Bool(_) => todo!(),
            CopyableValue::Void(_) => todo!(),
            CopyableValue::EnumKind(enum_kind) => todo!(),
        },
        TempValue::EvalOwned(_) => todo!(),
        TempValue::EvalPure(_) => todo!(),
        TempValue::EvalRef(_) => todo!(),
        TempValue::FullyOwnedRef(value) => todo!(),
        TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
        TempValue::TempOwned(_) => todo!(),
        TempValue::PartiallyOwnedRef(_) => todo!(),
        TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
    }
}
