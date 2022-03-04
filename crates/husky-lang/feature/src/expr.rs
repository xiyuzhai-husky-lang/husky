use std::sync::Arc;

use file::FilePtr;
use scope::{RangedScope, ScopePtr};
use semantics::{
    DeclStmt, EntityKind, Expr, ExprKind, ImprStmt, InstructionSheet, Opn, SemanticQueryGroup,
};
use syntax_types::InputPlaceholder;
use text::TextRange;
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
        opr: BinaryOpr,
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
        db: &dyn SemanticQueryGroup,
        expr: &Expr,
        symbols: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> Arc<Self> {
        Arc::new(match expr.kind {
            ExprKind::Variable(varname) => symbols
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
            ExprKind::Scope { scope, compiled } => todo!(),
            ExprKind::Literal(value) => FeatureExpr {
                kind: FeatureExprKind::Literal(value),
                feature: features.alloc(Feature::Literal(value)),
                range: expr.range,
                file: expr.file,
                eval_id: Default::default(),
            },
            ExprKind::Bracketed(_) => todo!(),
            ExprKind::Opn {
                opn,
                compiled,
                ref opds,
            } => match opn {
                Opn::Binary { opr, this, kind } => match this {
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
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(_) => todo!(),
                Opn::RoutineCall(routine) => {
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
                Opn::PattCall => todo!(),
                Opn::MembVarAccess => todo!(),
                Opn::MembFuncCall(_) => todo!(),
                Opn::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        })
    }
}
