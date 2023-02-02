// mod always_arrived;
// mod can_have_subtraces;
// mod file_range;
// mod impl_reachable;
// mod impl_stats;
// mod kind;
// mod stalk;
// mod subtraces;
// #[cfg(test)]
// mod tests;

// use husky_comptime::*;
// use husky_eager_semantics::*;
// use husky_entity_path::EntityPath;
// use husky_entity_semantics::*;
// use husky_feature_eval::EvalFeature;
// use husky_feature_gen::*;
// use EntityPath;
// use husky_term::Term;
// use husky_doc::TextRange;
// use husky_trace_protocol::*;
// use husky_vm::{History, HistoryEntry, InstructionSheet, LoopFrameData, VMConditionBranch};
// use serde::Serialize;
// use std::sync::Arc;

// // ts: { idx: number; parent: number | null; tokens: Token[] }
// #[derive(Debug)]
// pub struct Trace {
//     pub variant: TraceVariant,
//     pub raw_data: TraceData,
//     pub range: TextRange,
//     pub file: DiffPath,
// }
// #[derive(Debug)]
// pub enum TraceVariant {
//     Main(FeatureRepr),
//     Module {
//         entity_path: EntityPath,
//         file: DiffPath,
//         range: TextRange,
//     },
//     EntityFeature {
//         entity_path: EntityPath,
//         repr: FeatureRepr,
//     },
//     FeatureStmt(Arc<FeatureLazyStmt>),
//     FeatureBranch(Arc<FeatureLazyBranch>),
//     FeatureExpr(Arc<FeatureLazyExpr>),
//     FeatureCallArgument {
//         name: &'static str,
//         argument: Arc<FeatureLazyExpr>,
//     },
//     FuncStmt {
//         stmt: Arc<FuncStmt>,
//         history: Arc<History<'static>>,
//     },
//     ProcStmt {
//         stmt: Arc<ProcStmt>,
//         history: Arc<History<'static>>,
//     },
//     ProcBranch {
//         stmt: Arc<ProcStmt>,
//         branch: Arc<ProcConditionFlowBranch>,
//         opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
//         branch_idx: u8,
//         history: Arc<History<'static>>,
//     },
//     FuncBranch {
//         stmt: Arc<FuncStmt>,
//         branch: Arc<FuncConditionFlowBranch>,
//         opt_vm_branch: Option<Arc<VMConditionBranch>>, // not none when executed
//         branch_idx: u8,
//         history: Arc<History<'static>>,
//     },
//     LoopFrame {
//         loop_stmt: Arc<ProcStmt>,
//         body_instruction_sheet: Arc<InstructionSheet>,
//         body_stmts: Arc<Vec<Arc<ProcStmt>>>,
//         loop_frame_data: LoopFrameData<'static>,
//     },
//     EagerExpr {
//         expr: Arc<EagerExpr>,
//         history: Arc<History<'static>>,
//     },
//     EagerCallArgument {
//         name: &'static str,
//         argument: Arc<EagerExpr>,
//         history: Arc<History<'static>>,
//     },
//     CallHead {
//         entity: Arc<EntityDefn>,
//     },
// }

// impl TraceVariant {
//     pub fn input(db: &dyn FeatureGenQueryGroup) -> Self {
//         todo!()
//         // TraceVariant::EntityFeature {
//         //     entity_path: db.intern_entity_route(EntityRoute {
//         //         variant: EntityRouteVariant::TargetInputValue,
//         //         temporal_arguments: Default::default(),
//         //         spatial_arguments: Default::default(),
//         //     }),
//         //     repr: FeatureRepr::input(db),
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
