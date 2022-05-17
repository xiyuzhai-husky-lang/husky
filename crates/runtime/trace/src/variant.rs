use feature::*;
use vm::{
    History, HistoryEntry, InstructionSheet, LoopFrameData, StackValueSnapshot, VMBranch, VMControl,
};
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone)]
pub enum TraceVariant<'eval> {
    Main(Arc<FeatureBlock>),
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    FeatureCallInput {
        ident: CustomIdentifier,
        input: Arc<FeatureExpr>,
    },
    FuncStmt {
        stmt: Arc<FuncStmt>,
        history: Arc<History<'eval>>,
    },
    ProcStmt {
        stmt: Arc<ProcStmt>,
        history: Arc<History<'eval>>,
    },
    ProcBranch {
        stmt: Arc<ProcStmt>,
        branch: Arc<ProcBranch>,
        opt_vm_branch: Option<Arc<VMBranch>>, // not none when executed
        branch_idx: u8,
        history: Arc<History<'eval>>,
    },
    LoopFrame {
        loop_stmt: Arc<ProcStmt>,
        body_instruction_sheet: Arc<InstructionSheet>,
        body_stmts: Arc<Vec<Arc<ProcStmt>>>,
        loop_frame_data: LoopFrameData<'eval>,
    },
    EagerExpr {
        expr: Arc<EagerExpr>,
        history: Arc<History<'eval>>,
    },
    CallHead {
        entity: Arc<EntityDefn>,
        tokens: Vec<TokenProps<'eval>>,
    },
}

impl<'eval> TraceVariant<'eval> {
    pub fn tag(&self) -> &'static str {
        match self {
            TraceVariant::Main(_) => "TraceVariant::Main",
            TraceVariant::FeatureStmt(_) => "TraceVariant::FeatureStmt",
            TraceVariant::FeatureBranch(_) => "TraceVariant::FeatureBranch",
            TraceVariant::FeatureExpr(_) => "TraceVariant::FeatureExpr",
            TraceVariant::FeatureCallInput { .. } => "TraceVariant::FeatureCallInput",
            TraceVariant::FuncStmt { .. } => "TraceVariant::FuncStmt",
            TraceVariant::ProcStmt { .. } => "TraceVariant::ProcStmt",
            TraceVariant::ProcBranch { .. } => "TraceVariant::ProcBranch",
            TraceVariant::LoopFrame { .. } => "TraceVariant::ProcBranch",
            TraceVariant::EagerExpr { .. } => "TraceVariant::EagerExpr",
            TraceVariant::CallHead { .. } => "TraceVariant::CallHead",
        }
    }

    pub fn file_and_range(&self) -> (FilePtr, TextRange) {
        match self {
            TraceVariant::Main(ref block) => (block.file, block.range),
            TraceVariant::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceVariant::FeatureExpr(ref expr) => (expr.expr.file, expr.expr.range),
            TraceVariant::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceVariant::FeatureCallInput { input, .. } => (input.expr.file, input.expr.range),
            TraceVariant::FuncStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::EagerExpr { ref expr, .. } => (expr.file, expr.range),
            TraceVariant::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceVariant::ProcStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
            TraceVariant::ProcBranch { stmt, branch, .. } => (stmt.file, branch.range),
        }
    }

    pub fn has_subtraces(&self, reachable: bool) -> bool {
        if !reachable {
            return false;
        }
        match self {
            TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. } => false,
            TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
                ProcStmtVariant::Init { .. }
                | ProcStmtVariant::Assert { .. }
                | ProcStmtVariant::Execute { .. }
                | ProcStmtVariant::Return { .. } => false,
                ProcStmtVariant::Loop { .. } => true,
                ProcStmtVariant::ConditionFlow { .. } => panic!(),
                ProcStmtVariant::Break => false,
            },
            TraceVariant::LoopFrame { .. }
            | TraceVariant::Main(_)
            | TraceVariant::FeatureBranch(_) => true,
            TraceVariant::FeatureExpr(ref expr) => match expr.kind {
                FeatureExprKind::PrimitiveLiteral(_)
                | FeatureExprKind::PrimitiveBinaryOpr { .. }
                | FeatureExprKind::Variable { .. } => false,
                FeatureExprKind::StructOriginalFieldAccess { .. } => todo!(),
                FeatureExprKind::EnumLiteral { .. } => todo!(),
                FeatureExprKind::EntityFeature { .. } => todo!(),
                FeatureExprKind::NewRecord { ty, ref opds, .. } => todo!(),
                FeatureExprKind::RecordOriginalFieldAccess {
                    ref this,
                    field_ident,
                    ..
                } => todo!(),
                FeatureExprKind::This { ref repr } => todo!(),
                FeatureExprKind::GlobalInput => false,
                FeatureExprKind::RoutineCall {
                    ref routine_defn, ..
                } => !routine_defn.is_builtin(),
                FeatureExprKind::PatternCall {} => true,
                FeatureExprKind::RecordDerivedFieldAccess { .. } => todo!(),
                FeatureExprKind::ElementAccess { ref opds, .. } => false,
            },
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => match expr.variant {
                EagerExprVariant::Variable(_)
                | EagerExprVariant::EntityRoute { .. }
                | EagerExprVariant::PrimitiveLiteral(_) => false,
                EagerExprVariant::Bracketed(_) => todo!(),
                EagerExprVariant::Opn {
                    ref opn_variant,
                    ref opds,
                    ..
                } => match opn_variant {
                    EagerOpnVariant::RoutineCall(ranged_route) => !ranged_route.route.is_builtin(),
                    EagerOpnVariant::TypeCall { ranged_ty, .. } => !ranged_ty.route.is_builtin(),
                    EagerOpnVariant::PatternCall => todo!(),
                    EagerOpnVariant::FieldAccess { .. } => false,
                    EagerOpnVariant::Binary { .. }
                    | EagerOpnVariant::Prefix { .. }
                    | EagerOpnVariant::Suffix { .. }
                    | EagerOpnVariant::MethodCall { .. } => !opds[0].ty.is_builtin(),
                    EagerOpnVariant::ElementAccess => false,
                },
                EagerExprVariant::Lambda(_, _) => todo!(),
                EagerExprVariant::This => todo!(),
            },
            TraceVariant::CallHead { .. } => false,
            TraceVariant::ProcBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::BranchGroup {
                            opt_branch_entered: branch_entered,
                            ..
                        } => {
                            if *branch_entered == Some(*branch_idx) {
                                true
                            } else {
                                false
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    false
                }
            }
        }
    }

    pub fn reachable(&self) -> bool {
        match self {
            TraceVariant::Main(_) => true,
            TraceVariant::FeatureStmt(_) => true,
            TraceVariant::FeatureBranch(_) => true,
            TraceVariant::FeatureExpr(_) => true,
            TraceVariant::FeatureCallInput { ident, input } => true,
            TraceVariant::FuncStmt { stmt, history } => match stmt.variant {
                FuncStmtVariant::Init {
                    ref initial_value, ..
                } => history.contains(initial_value),
                FuncStmtVariant::Assert { ref condition } => history.contains(condition),
                FuncStmtVariant::Return { ref result } => history.contains(result),
                FuncStmtVariant::ConditionFlow { .. } => todo!(),
            },
            TraceVariant::ProcStmt { stmt, history } => match stmt.variant {
                ProcStmtVariant::Init {
                    ref initial_value, ..
                } => history.contains(initial_value),
                ProcStmtVariant::Assert { ref condition } => history.contains(condition),
                ProcStmtVariant::Execute { ref expr } => history.contains(expr),
                ProcStmtVariant::ConditionFlow { .. } => panic!(),
                ProcStmtVariant::Loop { .. } | ProcStmtVariant::Break => history.contains(stmt),
                ProcStmtVariant::Return { ref result } => history.contains(result),
            },
            TraceVariant::LoopFrame {
                loop_stmt,
                body_instruction_sheet,
                body_stmts,
                loop_frame_data,
            } => true,
            TraceVariant::EagerExpr { expr, history } => history.contains(expr),
            TraceVariant::CallHead { entity, tokens } => true,
            TraceVariant::ProcBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::BranchGroup {
                            opt_branch_entered, ..
                        } => {
                            if let Some(branch_entered) = opt_branch_entered {
                                if branch_idx > branch_entered {
                                    false
                                } else {
                                    true
                                }
                            } else {
                                false
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    false
                }
            }
        }
    }
}

impl<'eval> Serialize for TraceVariant<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            TraceVariant::Main(_) => "Main",
            TraceVariant::FeatureStmt(_) => "FeatureStmt",
            TraceVariant::FeatureBranch(_) => "FeatureBranch",
            TraceVariant::FeatureExpr(_) => "FeatureExpr",
            TraceVariant::FeatureCallInput { .. } => "FeatureCallInput",
            TraceVariant::FuncStmt { .. } => "FuncStmt",
            TraceVariant::ProcStmt { .. } => "ProcStmt",
            TraceVariant::EagerExpr { .. } => "EagerExpr",
            TraceVariant::CallHead { .. } => "CallHead",
            TraceVariant::LoopFrame { .. } => "LoopFrame",
            TraceVariant::ProcBranch { .. } => "ProcBranch",
        })
    }
}
