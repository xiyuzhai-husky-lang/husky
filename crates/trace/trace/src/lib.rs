mod impl_can_have_subtraces;
mod impl_file_range;
mod impl_kind;
mod impl_reachable;
mod impl_stats;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;

use fold::Indent;
use husky_eager_semantics::*;
use husky_entity_route::EntityRoutePtr;
use husky_entity_semantics::*;
use husky_feature_eval::EvalFeature;
use husky_feature_gen::*;
use husky_feature_gen::*;
use husky_file::FilePtr;
use husky_print_utils::p;
use husky_text::{HuskyText, TextRange};
use husky_trace_protocol::*;
use husky_word::CustomIdentifier;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};
use vm::{History, HistoryEntry, InstructionSheet, LoopFrameData, VMConditionBranch, VMControl};

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug)]
pub struct Trace {
    pub variant: TraceVariant<'static>,
    pub raw_data: TraceData,
    pub range: TextRange,
    pub file: FilePtr,
}
#[derive(Debug)]
pub enum TraceVariant<'eval> {
    Main(FeatureRepr),
    Module {
        route: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
    },
    EntityFeature {
        route: EntityRoutePtr,
        repr: FeatureRepr,
    },
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    FeatureCallArgument {
        name: &'static str,
        argument: Arc<FeatureExpr>,
    },
    FuncStmt {
        stmt: Arc<FuncStmt>,
        history: Arc<History<'static>>,
    },
    ProcStmt {
        stmt: Arc<ProcStmt>,
        history: Arc<History<'static>>,
    },
    ProcBranch {
        stmt: Arc<ProcStmt>,
        branch: Arc<ProcConditionFlowBranch>,
        opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
        branch_idx: u8,
        history: Arc<History<'static>>,
    },
    FuncBranch {
        stmt: Arc<FuncStmt>,
        branch: Arc<FuncConditionFlowBranch>,
        opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
        branch_idx: u8,
        history: Arc<History<'static>>,
    },
    LoopFrame {
        loop_stmt: Arc<ProcStmt>,
        body_instruction_sheet: Arc<InstructionSheet>,
        body_stmts: Arc<Vec<Arc<ProcStmt>>>,
        loop_frame_data: LoopFrameData<'eval>,
    },
    EagerExpr {
        expr: Arc<EagerExpr>,
        history: Arc<History<'static>>,
    },
    EagerCallArgument {
        name: &'static str,
        argument: Arc<EagerExpr>,
        history: Arc<History<'static>>,
    },
    CallHead {
        entity: Arc<EntityDefn>,
        tokens: Vec<TraceTokenData>,
    },
}

impl<'eval> Serialize for TraceVariant<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.kind().serialize(serializer)
    }
}

impl PartialEq for Trace {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Trace {}

impl Trace {
    pub fn id(&self) -> TraceId {
        self.raw_data.id
    }
}
