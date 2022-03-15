use semantics::OpnKind;

use crate::*;

impl Trace {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        match self.kind {
            TraceKind::Main(_)
            | TraceKind::LazyStmt(_)
            | TraceKind::LazyBranch(_)
            | TraceKind::Input(_)
            | TraceKind::StrictDeclStmt { .. }
            | TraceKind::ImprStmt { .. }
            | TraceKind::LoopFrame { .. }
            | TraceKind::CallHead { .. } => None,
            TraceKind::LazyExpr(ref expr) => match expr.kind {
                feature::LazyExprKind::Literal(_)
                | feature::LazyExprKind::PrimitiveBinaryOpr { .. }
                | feature::LazyExprKind::Variable { .. } => None,
                feature::LazyExprKind::FuncCall { .. } | feature::LazyExprKind::ProcCall { .. } => {
                    Some(SubtracesContainerClass::Call)
                }
            },
            TraceKind::StrictExpr { ref expr, .. } => match expr.kind {
                semantics::ExprKind::Variable(_)
                | semantics::ExprKind::Scope { .. }
                | semantics::ExprKind::Literal(_) => None,
                semantics::ExprKind::Opn {
                    opn_kind: opn,
                    ref opds,
                    ..
                } => match opn {
                    OpnKind::MembVarAccess | OpnKind::ElementAccess => None,
                    OpnKind::Binary { .. } | OpnKind::Prefix(_) | OpnKind::Suffix(_) => {
                        if opds[0].ty.is_builtin() {
                            None
                        } else {
                            Some(SubtracesContainerClass::Call)
                        }
                    }
                    OpnKind::RoutineCall { .. } | OpnKind::MembFuncCall(_) => {
                        Some(SubtracesContainerClass::Call)
                    }
                    OpnKind::PattCall => panic!(),
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
