use std::sync::Arc;

use file::FilePtr;
use scope::ScopePtr;
use semantics::{EntityVersionControl, Expr, ExprKind, Opn};
use text::TextRange;
use word::BuiltinIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureExpr {
    pub kind: FeatureExprKind,
    pub(crate) feature: FeaturePtr,
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
        func: ScopePtr,
        scope_expr_range: TextRange,
        uid: EntityUid,
        inputs: Vec<Arc<FeatureExpr>>,
        compiled: Option<()>,
    },
}

impl FeatureExpr {
    pub fn new(
        vc: &EntityVersionControl,
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
                        let lopd = Self::new(vc, &opds[0], symbols, features);
                        let ropd = Self::new(vc, &opds[1], symbols, features);
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
                        }
                    }
                    _ => todo!(),
                },
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(_) => todo!(),
                Opn::FuncCall {
                    func,
                    scope_expr_range,
                } => {
                    let uid = vc.uid(func);
                    let inputs: Vec<_> = opds
                        .iter()
                        .map(|opd| Self::new(vc, opd, symbols, features))
                        .collect();
                    let feature = features.alloc(Feature::FuncCall {
                        func,
                        uid,
                        inputs: inputs.iter().map(|expr| expr.feature).collect(),
                    });
                    Self {
                        kind: FeatureExprKind::FuncCall {
                            func,
                            scope_expr_range,
                            inputs,
                            uid,
                            compiled: None,
                        },
                        feature,
                        range: expr.range,
                        file: expr.file,
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
