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
    FieldAccessInterpreted {
        field_idx: u8,
        field_binding: Binding,
    },
    CallLinkage {
        linkage: Linkage,
    },
    CallInterpreted {
        routine_uid: EntityUid,
        nargs: u8,
        has_this: bool,
    },
    NewVirtualStruct {
        fields: Vec<CustomIdentifier>,
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

pub fn binary_assign<'temp, 'eval>(
    opt_binary_opr: Option<PureBinaryOpr>,
    lopd: &mut TempValue<'temp, 'eval>,
    ropd: TempValue<'temp, 'eval>,
) -> VMRuntimeResult<()> {
    match lopd {
        TempValue::CopyableOrTempMutEval { ref mut value, .. } => {
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

pub fn incr<'temp, 'eval>(opd: &mut TempValue<'temp, 'eval>) {
    let opd_primitive = opd.take_copyable();
    match opd {
        TempValue::CopyableOrTempMutEval { value, owner, gen } => {
            value.assign(TempValue::Copyable(match opd_primitive {
                CopyableValue::I32(i) => (i + 1).into(),
                CopyableValue::F32(_) => todo!(),
                _ => panic!(),
            }))
        }

        _ => panic!(),
    }
}

pub fn decr<'temp, 'eval>(opd: &mut TempValue<'temp, 'eval>) {
    let opd_primitive = opd.take_copyable();
    match opd {
        TempValue::CopyableOrTempMutEval { value, owner, gen } => {
            value.assign(TempValue::Copyable(match opd_primitive {
                CopyableValue::I32(i) => (i - 1).into(),
                CopyableValue::F32(_) => todo!(),
                _ => panic!(),
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
    pub fn act_on<'temp, 'eval>(self, opd: TempValue<'temp, 'eval>) -> TempValue<'temp, 'eval> {
        match self {
            CastOpn::AsI32 => TempValue::Copyable(cast_as_i32(opd).into()),
            CastOpn::AsB32 => TempValue::Copyable(cast_as_b32(opd).into()),
            CastOpn::AsF32 => TempValue::Copyable(cast_as_f32(opd).into()),
        }
    }
}

fn cast_as_i32<'temp, 'eval>(opd: TempValue<'temp, 'eval>) -> i32 {
    match opd {
        TempValue::Moved => todo!(),
        TempValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i,
            CopyableValue::B32(b) => b as i32,
            CopyableValue::Bool(b) => b as i32,
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as i32,
            _ => panic!(),
        },
        TempValue::EvalRef(_) => todo!(),
        TempValue::CopyableOrTempMutEval { value, owner, gen } => todo!(),
        _ => panic!(),
    }
}

fn cast_as_b32<'temp, 'eval>(opd: TempValue<'temp, 'eval>) -> u32 {
    match opd {
        TempValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i as u32,
            CopyableValue::B32(b) => b,
            CopyableValue::EnumKind(enum_kind) => enum_kind.kind_idx as u32,
            _ => panic!(),
        },
        TempValue::Moved => todo!(),
        TempValue::OwnedEval(_) => todo!(),
        TempValue::EvalRef(_) => todo!(),
        TempValue::TempRefEval(value) => todo!(),
        TempValue::CopyableOrTempMutEval { value, owner, gen } => todo!(),
        _ => panic!(),
    }
}

fn cast_as_f32<'temp, 'eval>(opd: TempValue<'temp, 'eval>) -> f32 {
    match opd {
        TempValue::Moved => todo!(),
        TempValue::Copyable(value) => match value {
            CopyableValue::I32(i) => i as f32,
            CopyableValue::F32(f) => f,
            _ => panic!(),
        },
        TempValue::EvalRef(_) => todo!(),
        TempValue::TempRefEval(value) => todo!(),
        TempValue::CopyableOrTempMutEval { value, owner, gen } => todo!(),
        _ => panic!(),
    }
}
