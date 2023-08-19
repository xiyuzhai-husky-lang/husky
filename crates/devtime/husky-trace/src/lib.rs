mod always_arrived;
mod can_have_subtraces;
mod file_range;
mod impl_reachable;
mod impl_stats;
mod kind;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;

use husky_entity_path::EntityPath;
use husky_ethereal_term::EtherealTerm;
use husky_hir_eager_expr::*;
use husky_hir_lazy_expr::*;
use husky_text::TextRange;
use husky_trace_protocol::*;
use husky_val_repr::{db::ValReprDb, *};
use husky_vm::{History, HistoryEntry, Instructions, LoopFrameData, VMConditionBranch};
use serde::Serialize;
use std::sync::Arc;

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug)]
pub struct Trace {
    pub variant: TraceVariant,
    pub raw_data: TraceData,
    pub range: TextRange,
    // pub file: DiffPath,
}

#[derive(Debug)]
pub enum TraceVariant {
    Main(
        // ValRepr
    ),
    Module {
        item_path: EntityPath,
        // file: DiffPath,
        range: TextRange,
    },
    EntityFeature {
        item_path: EntityPath,
        // repr: ValRepr,
    },
    FeatureStmt(Arc<ValStmt>),
    LazyBranch(
        // Arc<FeatureLazyBranch>
    ),
    FeatureExpr(ValExpr),
    FeatureCallArgument {
        name: &'static str,
        argument: ValExpr,
    },
    EagerStmt {
        eager_expr_region: HirEagerExprRegion,
        stmt: HirEagerStmtIdx,
        history: Arc<History>,
    },
    EagerBranch {
        stmt: HirEagerStmtIdx,
        // branch: Arc<ProcConditionFlowBranch>,
        opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
        branch_idx: u8,
        history: Arc<History>,
    },
    FuncBranch {
        stmt: HirEagerStmtIdx,
        // branch: Arc<FuncConditionFlowBranch>,
        opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
        branch_idx: u8,
        history: Arc<History>,
    },
    LoopFrame {
        loop_stmt: HirEagerStmtIdx,
        body_instruction_sheet: Instructions,
        body_stmts: Arc<Vec<HirEagerStmtIdx>>,
        loop_frame_data: LoopFrameData,
    },
    EagerExpr {
        expr: HirEagerExprIdx,
        history: Arc<History>,
    },
    EagerCallArgument {
        name: &'static str,
        argument: HirEagerExprIdx,
        history: Arc<History>,
    },
    CallHead {
        // item: Arc<EntityDefn>,
    },
}

impl TraceVariant {
    pub fn input(db: &dyn ValReprDb) -> Self {
        todo!()
        // TraceVariant::EntityFeature {
        //     item_path: db.intern_item_route(EntityRoute {
        //         variant: EntityRouteVariant::TargetInputValue,
        //         temporal_arguments: Default::default(),
        //         spatial_arguments: Default::default(),
        //     }),
        //     repr: ValRepr::input(db),
        // }
    }
}

impl Serialize for TraceVariant {
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
