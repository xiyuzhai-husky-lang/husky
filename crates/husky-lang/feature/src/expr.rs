use std::sync::Arc;

use itertools::Itertools;
use scope::ScopePtr;
use semantics::{Expr, ExprKind, Opn};
use vm::Compiled;
use word::BuiltinIdentifier;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureExpr {
    kind: FeatureExprKind,
    pub(crate) feature: FeaturePtr,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureExprKind {
    Literal(PrimitiveValue),
    PrimitiveBinaryOpr {
        opr: BinaryOpr,
        lopd: Arc<FeatureExpr>,
        ropd: Arc<FeatureExpr>,
    },
    Variable(CustomIdentifier),
}

impl FeatureExpr {
    pub fn new(expr: &Expr, symbols: &[FeatureSymbol], features: &FeatureUniqueAllocator) -> Self {
        match expr.kind {
            ExprKind::Variable(varname) => symbols
                .iter()
                .rev()
                .find_map(|symbol| {
                    if symbol.varname == varname {
                        Some(Self {
                            kind: FeatureExprKind::Variable(varname),
                            feature: symbol.feature,
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
                        let lopd = Arc::new(Self::new(&opds[0], symbols, features));
                        let ropd = Arc::new(Self::new(&opds[0], symbols, features));
                        let feature = features.alloc(Feature::PrimitiveBinaryOpr {
                            opr,
                            lopd: lopd.feature,
                            ropd: ropd.feature,
                        });
                        Self {
                            kind: FeatureExprKind::PrimitiveBinaryOpr { opr, lopd, ropd },
                            feature,
                        }
                    }
                    _ => todo!(),
                },
                Opn::Prefix(_) => todo!(),
                Opn::Suffix(_) => todo!(),
                Opn::FuncCall { func } => todo!(),
                Opn::PattCall => todo!(),
                Opn::MembVarAccess => todo!(),
                Opn::MembFuncCall(_) => todo!(),
                Opn::ElementAccess => todo!(),
            },
            ExprKind::Lambda(_, _) => todo!(),
        }
    }
}
