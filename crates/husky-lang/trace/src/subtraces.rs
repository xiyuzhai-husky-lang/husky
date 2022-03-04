use semantics::Opn;

use crate::*;

impl Trace {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        match self.kind {
            TraceKind::Main(_)
            | TraceKind::FeatureStmt(_)
            | TraceKind::FeatureBranch(_)
            | TraceKind::Input(_)
            | TraceKind::StrictDeclStmt { .. }
            | TraceKind::ImprStmt { .. }
            | TraceKind::CallHead { .. } => None,
            TraceKind::FeatureExpr(ref expr) => match expr.kind {
                feature::FeatureExprKind::Literal(_)
                | feature::FeatureExprKind::PrimitiveBinaryOpr { .. }
                | feature::FeatureExprKind::Variable { .. } => None,
                feature::FeatureExprKind::FuncCall { .. }
                | feature::FeatureExprKind::ProcCall { .. } => Some(SubtracesContainerClass::Call),
            },
            TraceKind::Expr { ref expr, .. } => match expr.kind {
                semantics::ExprKind::Variable(_)
                | semantics::ExprKind::Scope { .. }
                | semantics::ExprKind::Literal(_) => None,
                semantics::ExprKind::Opn { opn, ref opds, .. } => match opn {
                    Opn::MembVarAccess | Opn::ElementAccess => None,
                    Opn::Binary { .. } | Opn::Prefix(_) | Opn::Suffix(_) => {
                        if opds[0].ty.is_builtin() {
                            None
                        } else {
                            Some(SubtracesContainerClass::Call)
                        }
                    }
                    Opn::RoutineCall { .. } | Opn::MembFuncCall(_) => {
                        Some(SubtracesContainerClass::Call)
                    }
                    Opn::PattCall => panic!(),
                },
                semantics::ExprKind::Lambda(_, _) => todo!(),
                semantics::ExprKind::Bracketed(_) => panic!(),
            },
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
