use semantics::Opn;

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
                semantics::StrictExprKind::Variable(_)
                | semantics::StrictExprKind::Scope { .. }
                | semantics::StrictExprKind::Literal(_) => None,
                semantics::StrictExprKind::Opn { opn, ref opds, .. } => match opn {
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
                semantics::StrictExprKind::Lambda(_, _) => todo!(),
                semantics::StrictExprKind::Bracketed(_) => panic!(),
            },
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}
