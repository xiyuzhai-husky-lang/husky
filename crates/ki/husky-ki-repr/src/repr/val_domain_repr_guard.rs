use husky_hir_lazy_expr::{helpers::control_flow::HasControlFlow, HirLazyExprIdx, HirLazyStmtIdx};

use super::*;

#[derive(Clone)]
pub(crate) struct ValDomainReprGuard<'a> {
    db: &'a ::salsa::Db,
    parent_ki_repr: KiRepr,
    val_domain_repr: ValDomainRepr,
}

impl<'a> ValDomainReprGuard<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        parent_ki_repr: KiRepr,
        val_domain_repr: ValDomainRepr,
    ) -> Self {
        Self {
            db,
            parent_ki_repr,
            val_domain_repr,
        }
    }

    pub(crate) fn new_ki_repr(
        &mut self,
        source: KiReprExpansionSource,
        opn: ValOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
        has_control_flow: HasControlFlow,
    ) -> KiRepr {
        let ki_repr = KiRepr::new(
            self.val_domain_repr,
            opn,
            arguments,
            KiReprSource::Expansion {
                parent_ki_repr: self.parent_ki_repr,
                source,
            },
            self.db,
        );
        if has_control_flow.to_bool() {
            self.val_domain_repr = ValDomainRepr::ExprNotReturned(ki_repr)
        }
        ki_repr
    }

    pub(crate) fn new_expr_ki_repr(
        &mut self,
        expr: HirLazyExprIdx,
        opn: ValOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
        has_control_flow: HasControlFlow,
    ) -> KiRepr {
        let ki_repr = KiRepr::new(
            self.val_domain_repr,
            opn,
            arguments,
            KiReprSource::Expansion {
                parent_ki_repr: self.parent_ki_repr,
                source: KiReprExpansionSource::Expr { expr },
            },
            self.db,
        );
        if has_control_flow.to_bool() {
            self.val_domain_repr = ValDomainRepr::ExprNotReturned(ki_repr)
        }
        ki_repr
    }

    pub(crate) fn new_stmt_ki_repr(
        &mut self,
        stmt: HirLazyStmtIdx,
        opn: ValOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
    ) -> KiRepr {
        let ki_repr = KiRepr::new(
            self.val_domain_repr,
            opn,
            arguments,
            KiReprSource::Expansion {
                parent_ki_repr: self.parent_ki_repr,
                source: KiReprExpansionSource::Stmt { stmt },
            },
            self.db,
        );
        self.val_domain_repr = ValDomainRepr::StmtNotReturned(ki_repr);
        ki_repr
    }

    pub(crate) fn after_stmt(&mut self, ki_repr: KiRepr) {
        self.val_domain_repr = ValDomainRepr::StmtNotReturned(ki_repr)
    }

    // change self
    pub(crate) fn under_condition(&mut self, condition: KiRepr) -> ValDomainReprGuard<'a> {
        self.val_domain_repr = ValDomainRepr::ConditionNotSatisfied(condition);
        ValDomainReprGuard {
            db: self.db,
            parent_ki_repr: self.parent_ki_repr,
            val_domain_repr: ValDomainRepr::ConditionSatisfied(condition),
        }
    }
}
