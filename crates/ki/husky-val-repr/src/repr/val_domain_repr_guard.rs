use husky_hir_lazy_expr::{helpers::control_flow::HasControlFlow, HirLazyExprIdx, HirLazyStmtIdx};

use super::*;

#[derive(Clone)]
pub(crate) struct ValDomainReprGuard<'a> {
    db: &'a ::salsa::Db,
    parent_val_repr: ValRepr,
    val_domain_repr: ValDomainRepr,
}

impl<'a> ValDomainReprGuard<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        parent_val_repr: ValRepr,
        val_domain_repr: ValDomainRepr,
    ) -> Self {
        Self {
            db,
            parent_val_repr,
            val_domain_repr,
        }
    }

    pub(crate) fn new_val_repr(
        &mut self,
        source: ValReprExpansionSource,
        opn: ValOpn,
        arguments: SmallVec<[ValArgumentRepr; 4]>,
        has_control_flow: HasControlFlow,
    ) -> ValRepr {
        let val_repr = ValRepr::new(
            self.val_domain_repr,
            opn,
            arguments,
            ValReprSource::Expansion {
                parent_val_repr: self.parent_val_repr,
                source,
            },
            self.db,
        );
        if has_control_flow.to_bool() {
            self.val_domain_repr = ValDomainRepr::ExprNotReturned(val_repr)
        }
        val_repr
    }

    pub(crate) fn new_expr_val_repr(
        &mut self,
        expr: HirLazyExprIdx,
        opn: ValOpn,
        arguments: SmallVec<[ValArgumentRepr; 4]>,
        has_control_flow: HasControlFlow,
    ) -> ValRepr {
        let val_repr = ValRepr::new(
            self.val_domain_repr,
            opn,
            arguments,
            ValReprSource::Expansion {
                parent_val_repr: self.parent_val_repr,
                source: ValReprExpansionSource::Expr { expr },
            },
            self.db,
        );
        if has_control_flow.to_bool() {
            self.val_domain_repr = ValDomainRepr::ExprNotReturned(val_repr)
        }
        val_repr
    }

    pub(crate) fn new_stmt_val_repr(
        &mut self,
        stmt: HirLazyStmtIdx,
        opn: ValOpn,
        arguments: SmallVec<[ValArgumentRepr; 4]>,
    ) -> ValRepr {
        let val_repr = ValRepr::new(
            self.val_domain_repr,
            opn,
            arguments,
            ValReprSource::Expansion {
                parent_val_repr: self.parent_val_repr,
                source: ValReprExpansionSource::Stmt { stmt },
            },
            self.db,
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
            parent_val_repr: self.parent_val_repr,
            val_domain_repr: ValDomainRepr::ConditionSatisfied(condition),
        }
    }
}
