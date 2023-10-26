#![feature(trait_upcasting)]
mod always_arrived;
// mod can_have_subtraces;
pub mod db;
// mod impl_reachable;
// mod impl_stats;
// mod kind;
mod helpers;
mod node;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod trace;
mod utils;

pub use self::trace::*;

use self::db::*;
use husky_entity_path::EntityPath;
use husky_ethereal_term::EtherealTerm;
use husky_syn_decl::SynDecl;
use husky_syn_expr::*;
use husky_text_protocol::range::TextRange;
use husky_trace_protocol_old::*;
use husky_val_repr::{db::ValReprDb, *};
use husky_vfs::*;
use husky_vm::{History, HistoryEntry, Instructions, LoopFrameData, VMConditionBranch};
use serde::Serialize;
use std::sync::Arc;

// // ts: { idx: number; parent: number | null; tokens: TokenData[] }
// #[derive(Debug)]
// pub struct Trace {
//     pub variant: TraceVariant,
//     pub raw_data: TraceData,
//     pub range: TextRange,
//     pub file: DiffPath,
// }

// #[derive(Debug)]
// pub enum TraceVariant {
//     Main(
//         // ValRepr
//     ),
//     Module {
//         item_path: EntityPath,
//         // file: DiffPath,
//         range: TextRange,
//     },
//     EntityVal {
//         item_path: EntityPath,
//         // repr: ValRepr,
//     },
//     ValStmt(ValStmt),
//     ValBranch(ValBranch),
//     LazyExpr(ValExpr),
//     ValCallArgument {
//         name: &'static str,
//         argument: ValExpr,
//     },
//     EagerStmt {
//         eager_expr_region: SynExprRegion,
//         stmt: SynStmtIdx,
//         history: Arc<History>,
//     },
//     EagerBranch {
//         stmt: SynStmtIdx,
//         // branch: Arc<ProcConditionFlowBranch>,
//         opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
//         branch_idx: u8,
//         history: Arc<History>,
//     },
//     LoopFrame {
//         loop_stmt: SynStmtIdx,
//         body_instruction_sheet: Instructions,
//         body_stmts: Arc<Vec<SynStmtIdx>>,
//         loop_frame_data: LoopFrameData,
//     },
//     EagerExpr {
//         expr: SynExprIdx,
//         history: Arc<History>,
//     },
//     EagerCallArgument {
//         name: &'static str,
//         argument: SynExprIdx,
//         history: Arc<History>,
//     },
//     CallHead {
//         item: SynDecl,
//     },
// }

// impl TraceVariant {
//     pub fn input(db: &dyn ValReprDb) -> Self {
//         todo!()
//         // TraceVariant::EntityFeature {
//         //     item_path: db.intern_item_route(EntityRoute {
//         //         variant: EntityRouteVariant::TargetInputValue,
//         //         temporal_arguments: Default::default(),
//         //         spatial_arguments: Default::default(),
//         //     }),
//         //     repr: ValRepr::input(db),
//         // }
//     }
// }

// impl Serialize for TraceVariant {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.kind().serialize(serializer)
//     }
// }

// impl PartialEq for Trace {
//     fn eq(&self, other: &Self) -> bool {
//         self.id() == other.id()
//     }
// }

// impl Eq for Trace {}

// impl Trace {
//     pub fn id(&self) -> TraceId {
//         self.raw_data.id
//     }
// }

// pub trait TraceRuntime {}
