use std::sync::Arc;

use file::FilePtr;
use scope::InputPlaceholder;
use scope::{RangedScope, ScopePtr};
use semantics_eager::*;
use semantics_entity::*;
use semantics_lazy::*;
use text::TextRange;
use vm::{BinaryOpr, InstructionSheet};
use word::BuiltinIdentifier;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureExpr {
    pub kind: FeatureExprKind,
    pub(crate) feature: FeaturePtr,
    pub(crate) eval_id: FeatureEvalId,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureExprKind {
    Literal(PrimitiveValue),
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: Arc<FeatureExpr>,
        ropd: Arc<FeatureExpr>,
    },
    Variable {
        varname: CustomIdentifier,
        value: Arc<FeatureExpr>,
    },
    FuncCall {
        ranged_scope: RangedScope,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        inputs: Vec<Arc<FeatureExpr>>,
        compiled: Option<()>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<DeclStmt>>>,
    },
    ProcCall {
        ranged_scope: RangedScope,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        inputs: Vec<Arc<FeatureExpr>>,
        compiled: Option<()>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<ImprStmt>>>,
    },
}

impl FeatureExpr {
    pub fn new(
        db: &dyn EntityQueryGroup,
        expr: &LazyExpr,
        symbols: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> Arc<Self> {
        Arc::new(match expr.kind {
            LazyExprKind::Variable(varname) => symbols
                .iter()
                .rev()
                .find_map(|symbol| {
                    if symbol.varname == varname {
                        Some(Self {
                            kind: FeatureExprKind::Variable {
                                varname,
                                value: symbol.value.clone(),
                            },
                            feature: symbol.feature,
                            range: expr.range,
                            file: expr.file,
                            eval_id: Default::default(),
                        })
                    } else {
                        None
                    }
                })
                .unwrap(),
            LazyExprKind::Scope { scope, compiled } => todo!(),
            LazyExprKind::Literal(value) => FeatureExpr {
                kind: FeatureExprKind::Literal(value),
                feature: features.alloc(Feature::Literal(value)),
                range: expr.range,
                file: expr.file,
                eval_id: Default::default(),
            },
            LazyExprKind::Bracketed(_) => todo!(),
            LazyExprKind::Opn {
                opn_kind: opn,
                compiled,
                ref opds,
            } => match opn {
                LazyOpnKind::Binary { opr, this } => match this {
                    ScopePtr::Builtin(BuiltinIdentifier::Void)
                    | ScopePtr::Builtin(BuiltinIdentifier::I32)
                    | ScopePtr::Builtin(BuiltinIdentifier::F32)
                    | ScopePtr::Builtin(BuiltinIdentifier::B32)
                    | ScopePtr::Builtin(BuiltinIdentifier::B64) => {
                        let lopd = Self::new(db, &opds[0], symbols, features);
                        let ropd = Self::new(db, &opds[1], symbols, features);
                        let feature = features.alloc(Feature::PrimitiveBinaryOpr {
                            opr,
                            lopd: lopd.feature,
                            ropd: ropd.feature,
                        });
                        Self {
                            kind: FeatureExprKind::PrimitiveBinaryOpr { opr, lopd, ropd },
                            feature,
                            range: expr.range,
                            file: expr.file,
                            eval_id: Default::default(),
                        }
                    }
                    _ => todo!(),
                },
                LazyOpnKind::Prefix(_) => todo!(),
                LazyOpnKind::Suffix(_) => todo!(),
                LazyOpnKind::RoutineCall(routine) => {
                    let uid = db.entity_vc().uid(routine.scope);
                    let inputs: Vec<_> = opds
                        .iter()
                        .map(|opd| Self::new(db, opd, symbols, features))
                        .collect();
                    let feature = features.alloc(Feature::FuncCall {
                        func: routine.scope,
                        uid,
                        inputs: inputs.iter().map(|expr| expr.feature).collect(),
                    });
                    let entity = db.entity(routine.scope).unwrap();
                    let kind = match entity.kind() {
                        EntityKind::Func {
                            input_placeholders,
                            stmts,
                            ..
                        } => FeatureExprKind::FuncCall {
                            ranged_scope: routine,
                            uid,
                            compiled: None,
                            callee_file: entity.file,
                            input_placeholders: input_placeholders.clone(),
                            inputs,
                            instruction_sheet: db.instruction_sheet(routine.scope).unwrap(),
                            stmts: stmts.clone(),
                        },
                        EntityKind::Proc {
                            input_placeholders,
                            stmts,
                            ..
                        } => FeatureExprKind::ProcCall {
                            ranged_scope: routine,
                            uid,
                            compiled: None,
                            callee_file: entity.file,
                            input_placeholders: input_placeholders.clone(),
                            inputs,
                            instruction_sheet: db.instruction_sheet(routine.scope).unwrap(),
                            stmts: stmts.clone(),
                        },
                        _ => panic!(),
                    };
                    Self {
                        kind,
                        feature,
                        range: expr.range,
                        file: expr.file,
                        eval_id: Default::default(),
                    }
                }
                LazyOpnKind::PatternCall => todo!(),
                LazyOpnKind::MembVarAccess => todo!(),
                LazyOpnKind::MembFuncCall(_) => todo!(),
                LazyOpnKind::ElementAccess => todo!(),
                LazyOpnKind::TypeCall(_) => todo!(),
            },
            LazyExprKind::Lambda(_, _) => todo!(),
        })
    }
}
