use husky_hir_lazy_expr::helpers::control_flow::HasControlFlow;

use super::*;

#[derive(Clone)]
pub(crate) struct ValDomainReprGuard<'a> {
    db: &'a dyn ValReprDb,
    val_domain_repr: ValDomainRepr,
}

impl<'a> ValDomainReprGuard<'a> {
    pub(crate) fn new(db: &'a dyn ValReprDb, val_domain_repr: ValDomainRepr) -> Self {
        Self {
            db,
            val_domain_repr,
        }
    }

    pub(crate) fn new_expr_val_repr(
        &mut self,
        opn: ValOpn,
        arguments: SmallVec<[ValArgumentRepr; 4]>,
        _has_control_flow: HasControlFlow,
    ) -> ValRepr {
        let val_repr = ValRepr::new(
            self.db,
            self.val_domain_repr,
            opn,
            arguments,
            ValCachingClass::Stmt,
        );
        self.val_domain_repr = ValDomainRepr::StmtNotReturned(val_repr);
        val_repr
    }

    pub(crate) fn new_stmt_val_repr(
        &mut self,
        opn: ValOpn,
        arguments: SmallVec<[ValArgumentRepr; 4]>,
    ) -> ValRepr {
        let val_repr = ValRepr::new(
            self.db,
            self.val_domain_repr,
            opn,
            arguments,
            ValCachingClass::Stmt,
        );
        self.val_domain_repr = ValDomainRepr::StmtNotReturned(val_repr);
        val_repr
    }

    pub(crate) fn after_stmt(&mut self, val_repr: ValRepr) {
        self.val_domain_repr = ValDomainRepr::StmtNotReturned(val_repr)
    }

    // change self
    pub(crate) fn under_condition(&mut self, condition: ValRepr) -> ValDomainReprGuard<'a> {
        self.val_domain_repr = ValDomainRepr::ConditionNotSatisfied(condition);
        ValDomainReprGuard {
            db: self.db,
            val_domain_repr: ValDomainRepr::ConditionSatisfied(condition),
        }
    }
}
