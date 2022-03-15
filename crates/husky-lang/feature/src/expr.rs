use std::sync::Arc;

use file::FilePtr;
use scope::InputPlaceholder;
use scope::{RangedScope, ScopePtr};
use semantics::{DeclStmt, EntityKind, Expr, ExprKind, ImprStmt, OpnKind, SemanticQueryGroup};
use text::TextRange;
use vm::{BinaryOpr, InstructionSheet};
use word::BuiltinIdentifier;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LazyExpr {
    pub kind: LazyExprKind,
    pub(crate) feature: FeaturePtr,
    pub(crate) eval_id: FeatureEvalId,
    pub range: TextRange,
    pub file: FilePtr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LazyExprKind {
    Literal(PrimitiveValue),
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: Arc<LazyExpr>,
        ropd: Arc<LazyExpr>,
    },
    Variable {
        varname: CustomIdentifier,
        value: Arc<LazyExpr>,
    },
    FuncCall {
        ranged_scope: RangedScope,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        inputs: Vec<Arc<LazyExpr>>,
        compiled: Option<()>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<DeclStmt>>>,
    },
    ProcCall {
        ranged_scope: RangedScope,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        inputs: Vec<Arc<LazyExpr>>,
        compiled: Option<()>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<ImprStmt>>>,
    },
}

impl LazyExpr {
    pub fn new(
        db: &dyn SemanticQueryGroup,
        expr: &Expr,
        symbols: &[LazySymbol],
        features: &FeatureUniqueAllocator,
    ) -> Arc<Self> {
        Arc::new(match expr.kind {
            ExprKind::Variable(varname) => symbols
                .iter()
                .rev()
                .find_map(|symbol| {
                    if symbol.varname == varname {
                        Some(Self {
                            kind: LazyExprKind::Variable {
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
            ExprKind::Scope { scope, compiled } => todo!(),
            ExprKind::Literal(value) => LazyExpr {
                kind: LazyExprKind::Literal(value),
                feature: features.alloc(Feature::Literal(value)),
                range: expr.range,
                file: expr.file,
                eval_id: Default::default(),
            },
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn_kind: opn,
                compiled,
                ref opds,
            } => match opn {
                OpnKind::Binary { opr, this } => match this {
                    ScopePtr::Builtin(BuiltinIdentifier::Void)
                    | ScopePtr::Builtin(BuiltinIdentifier::I32)
                    | ScopePtr::Builtin(BuiltinIdentifier::F32)
                    | ScopePtr::Builtin(BuiltinIdentifier::B32)
                    | ScopePtr::Builtin(BuiltinIdentifier::B64) => {
                        let lopd = Self::new(db, &opds[0], symbols, features);
                        let ropd = Self::new(db, &opds[1], symbols, features);
                        let opr = match opr {
                            BinaryOpr::Pure(opr) => opr,
                            BinaryOpr::Assign(_) => todo!(),
                        };
                        let feature = features.alloc(Feature::PrimitiveBinaryOpr {
                            opr,
                            lopd: lopd.feature,
                            ropd: ropd.feature,
                        });
                        Self {
                            kind: LazyExprKind::PrimitiveBinaryOpr { opr, lopd, ropd },
                            feature,
                            range: expr.range,
                            file: expr.file,
                            eval_id: Default::default(),
                        }
                    }
                    _ => todo!(),
                },
                OpnKind::Prefix(_) => todo!(),
                OpnKind::Suffix(_) => todo!(),
                OpnKind::RoutineCall(routine) => {
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
                        } => LazyExprKind::FuncCall {
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
                        } => LazyExprKind::ProcCall {
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
                OpnKind::PattCall => todo!(),
                OpnKind::MembVarAccess => todo!(),
                OpnKind::MembFuncCall(_) => todo!(),
                OpnKind::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        })
    }
}
