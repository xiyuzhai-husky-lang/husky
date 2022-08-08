use husky_entity_route::EntityRoutePtr;
use husky_feature_gen::*;
use husky_word::CustomIdentifier;
use vm::{History, HistoryEntry, InstructionSheet, LoopFrameData, VMConditionBranch, VMControl};

use crate::*;

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

impl<'eval> TraceVariant<'eval> {
    pub fn kind(&self) -> TraceKind {
        match self {
            TraceVariant::Main(_) => TraceKind::Main,
            TraceVariant::Module { .. } => TraceKind::Module,
            TraceVariant::EntityFeature { ref repr, .. } => match repr.is_lazy() {
                true => TraceKind::EntityFeatureLazy,
                false => TraceKind::EntityFeatureEager,
            },
            TraceVariant::FeatureStmt(_) => TraceKind::FeatureStmt,
            TraceVariant::FeatureBranch(_) => TraceKind::FeatureBranch,
            TraceVariant::FeatureExpr(expr) => match expr.variant {
                FeatureExprVariant::StructDerivedLazyField { ref repr, .. } => {
                    match repr.is_lazy() {
                        true => TraceKind::FeatureExprLazy,
                        false => TraceKind::FeatureExprEager,
                    }
                }
                FeatureExprVariant::RecordDerivedField { .. } => TraceKind::FeatureExprLazy,
                FeatureExprVariant::ModelCall { .. } => TraceKind::FeatureExprLazy,
                FeatureExprVariant::EntityFeature { ref repr, .. } => match repr.is_lazy() {
                    true => TraceKind::FeatureExprLazy,
                    false => TraceKind::FeatureExprEager,
                },
                FeatureExprVariant::EvalInput => TraceKind::FeatureExprEager,
                FeatureExprVariant::NewRecord { .. } => TraceKind::FeatureExprLazy,
                FeatureExprVariant::NewVecFromList { .. } => TraceKind::FeatureExprEager,
                _ => TraceKind::FeatureExprEager,
            },
            TraceVariant::FeatureCallArgument { .. } => TraceKind::FeatureCallArgument,
            TraceVariant::FuncStmt { .. } => TraceKind::FuncStmt,
            TraceVariant::ProcStmt { .. } => TraceKind::ProcStmt,
            TraceVariant::ProcBranch { .. } => TraceKind::ProcBranch,
            TraceVariant::FuncBranch { .. } => TraceKind::FuncBranch,
            TraceVariant::LoopFrame { .. } => TraceKind::LoopFrame,
            TraceVariant::EagerExpr { .. } => TraceKind::EagerExpr,
            TraceVariant::CallHead { .. } => TraceKind::CallHead,
            TraceVariant::EagerCallArgument { .. } => TraceKind::EagerCallArgument,
        }
    }

    pub fn file_and_range(&self) -> (FilePtr, TextRange) {
        match self {
            TraceVariant::Main(ref repr) => (repr.file(), repr.text_range()),
            TraceVariant::Module { file, range, .. } => (*file, *range),
            TraceVariant::EntityFeature { ref repr, .. } => (repr.file(), repr.text_range()),
            TraceVariant::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceVariant::FeatureExpr(expr) => (expr.expr.file, expr.expr.range),
            TraceVariant::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceVariant::FeatureCallArgument { argument, .. } => {
                (argument.expr.file, argument.expr.range)
            }
            TraceVariant::FuncStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::EagerExpr { ref expr, .. } => (expr.file, expr.range),
            TraceVariant::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceVariant::ProcStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
            TraceVariant::ProcBranch { branch, .. } => (branch.file, branch.range),
            TraceVariant::FuncBranch { branch, .. } => (branch.file, branch.range),
            TraceVariant::EagerCallArgument { argument, .. } => (argument.file, argument.range),
        }
    }

    pub fn can_have_subtraces(&self, reachable: bool) -> bool {
        if !reachable {
            return false;
        }
        match self {
            TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureCallArgument { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::EagerCallArgument { .. } => false,
            TraceVariant::ProcStmt { ref stmt, .. } => match stmt.variant {
                ProcStmtVariant::Init { .. }
                | ProcStmtVariant::Assert { .. }
                | ProcStmtVariant::Execute { .. }
                | ProcStmtVariant::Return { .. } => false,
                ProcStmtVariant::Loop { .. } => true,
                ProcStmtVariant::ConditionFlow { .. } => panic!(),
                ProcStmtVariant::Break => false,
                ProcStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => todo!(),
            },
            TraceVariant::LoopFrame { .. }
            | TraceVariant::Main(_)
            | TraceVariant::Module { .. }
            | TraceVariant::EntityFeature { .. }
            | TraceVariant::FeatureBranch(_) => true,
            TraceVariant::FeatureExpr(expr) => match expr.variant {
                FeatureExprVariant::PrimitiveLiteral(_)
                | FeatureExprVariant::PrimitiveBinaryOpr { .. }
                | FeatureExprVariant::Variable { .. } => false,
                FeatureExprVariant::StructOriginalField { .. } => false,
                FeatureExprVariant::EnumKindLiteral { .. } => false,
                FeatureExprVariant::EntityFeature { .. } => true,
                FeatureExprVariant::NewRecord { ty, ref opds, .. } => todo!(),
                FeatureExprVariant::RecordOriginalField { .. } => false,
                FeatureExprVariant::ThisValue { .. } => false,
                FeatureExprVariant::EvalInput => false,
                FeatureExprVariant::RoutineCall {
                    ref routine_defn, ..
                } => !routine_defn.is_builtin(),
                FeatureExprVariant::RecordDerivedField { .. } => todo!(),
                FeatureExprVariant::ElementAccess { ref opds, .. } => false,
                FeatureExprVariant::StructDerivedLazyField {
                    ref this,
                    field_ident,
                    field_uid,
                    ref repr,
                } => true,
                FeatureExprVariant::ModelCall {
                    ref opds,
                    has_this,
                    ref model_defn,
                    ref internal,
                    ..
                } => match model_defn.variant {
                    EntityDefnVariant::Function { ref source, .. } => match source {
                        CallFormSource::Lazy { stmts } => true,
                        CallFormSource::Static(_) => false,
                        _ => panic!(),
                    },
                    _ => todo!(),
                },
                FeatureExprVariant::NewVecFromList { .. } => false,
                FeatureExprVariant::CustomBinaryOpr {
                    ref opt_instruction_sheet,
                    ..
                } => opt_instruction_sheet.is_some(),
            },
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => match expr.variant {
                EagerExprVariant::Variable { .. } | EagerExprVariant::PrimitiveLiteral(_) => false,
                EagerExprVariant::Bracketed(_) => todo!(),
                EagerExprVariant::Opn {
                    ref opn_variant,
                    ref opds,
                    ..
                } => match opn_variant {
                    EagerOpnVariant::RoutineCall(ranged_route) => !ranged_route.route.is_builtin(),
                    EagerOpnVariant::TypeCall { ranged_ty, .. } => !ranged_ty.route.is_builtin(),
                    EagerOpnVariant::Field { .. } => false,
                    EagerOpnVariant::Binary { .. }
                    | EagerOpnVariant::Prefix { .. }
                    | EagerOpnVariant::Suffix { .. }
                    | EagerOpnVariant::MethodCall { .. } => !opds[0].ty().is_builtin(),
                    EagerOpnVariant::Index { .. } => false,
                    EagerOpnVariant::NewVecFromList => todo!(),
                    EagerOpnVariant::ValueCall => todo!(),
                },
                EagerExprVariant::Lambda(_, _)
                | EagerExprVariant::ThisValue { .. }
                | EagerExprVariant::ThisField { .. }
                | EagerExprVariant::EnumKindLiteral(_) => false,
                EagerExprVariant::EntityFeature { .. } => true,
                EagerExprVariant::EntityFp { route } => todo!(),
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
                        HistoryEntry::ControlFlow {
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
            TraceVariant::FuncBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::ControlFlow {
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
            TraceVariant::Main(_)
            | TraceVariant::EntityFeature { .. }
            | TraceVariant::Module { .. } => true,
            TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureExpr(_) => true,
            TraceVariant::FeatureCallArgument { .. } | TraceVariant::EagerCallArgument { .. } => {
                true
            }
            TraceVariant::FuncStmt { stmt, history } => match stmt.variant {
                FuncStmtVariant::Init {
                    ref initial_value, ..
                } => history.contains(initial_value),
                FuncStmtVariant::Assert { ref condition } => history.contains(condition),
                FuncStmtVariant::Return { ref result, .. } => history.contains(result),
                FuncStmtVariant::ConditionFlow { .. } => panic!("FuncBranch"),
                FuncStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => history.contains(match_expr),
                FuncStmtVariant::Require { ref condition, .. } => history.contains(condition),
            },
            TraceVariant::ProcStmt { stmt, history } => match stmt.variant {
                ProcStmtVariant::Init {
                    ref initial_value, ..
                } => history.contains(initial_value),
                ProcStmtVariant::Assert { ref condition } => history.contains(condition),
                ProcStmtVariant::Execute { ref expr } => history.contains(expr),
                ProcStmtVariant::ConditionFlow { .. } => panic!("ProcBranch"),
                ProcStmtVariant::Loop { .. } | ProcStmtVariant::Break => history.contains(stmt),
                ProcStmtVariant::Return { ref result, .. } => history.contains(result),
                ProcStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => todo!(),
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
                        HistoryEntry::ControlFlow {
                            opt_branch_entered, ..
                        } => {
                            if let Some(branch_entered) = opt_branch_entered {
                                if branch_idx > branch_entered {
                                    false
                                } else {
                                    true
                                }
                            } else {
                                *branch_idx == 0
                            }
                        }
                        _ => panic!(),
                    }
                } else {
                    false
                }
            }
            TraceVariant::FuncBranch {
                stmt,
                branch_idx,
                history,
                ..
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::ControlFlow {
                            opt_branch_entered, ..
                        } => {
                            if let Some(branch_entered) = opt_branch_entered {
                                if branch_idx > branch_entered {
                                    false
                                } else {
                                    true
                                }
                            } else {
                                *branch_idx == 0
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
            TraceVariant::EntityFeature { .. } => "EntityFeature",
            TraceVariant::Module { .. } => "Module",
            TraceVariant::FeatureStmt(_) => "FeatureStmt",
            TraceVariant::FeatureBranch(_) => "FeatureBranch",
            TraceVariant::FeatureExpr(_) => "FeatureExpr",
            TraceVariant::FeatureCallArgument { .. } => "FeatureCallInput",
            TraceVariant::FuncStmt { .. } => "FuncStmt",
            TraceVariant::ProcStmt { .. } => "ProcStmt",
            TraceVariant::EagerExpr { .. } => "EagerExpr",
            TraceVariant::CallHead { .. } => "CallHead",
            TraceVariant::LoopFrame { .. } => "LoopFrame",
            TraceVariant::ProcBranch { .. } => "ProcBranch",
            TraceVariant::FuncBranch { .. } => "FuncBranch",
            TraceVariant::EagerCallArgument { .. } => "EagerCallArgument",
        })
    }
}
