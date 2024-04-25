use husky_hir_lazy_expr::{helpers::control_flow::HasControlFlow, HirLazyExprIdx, HirLazyStmtIdx};

use super::*;

#[derive(Clone)]
pub(crate) struct KiDomainReprGuard<'a> {
    db: &'a ::salsa::Db,
    parent_ki_repr: KiRepr,
    ki_domain_repr: KiDomainRepr,
}

impl<'a> KiDomainReprGuard<'a> {
    pub(crate) fn new(
        db: &'a ::salsa::Db,
        parent_ki_repr: KiRepr,
        ki_domain_repr: KiDomainRepr,
    ) -> Self {
        Self {
            db,
            parent_ki_repr,
            ki_domain_repr,
        }
    }

    pub(crate) fn new_ki_repr(
        &mut self,
        source: KiReprExpansionSource,
        opn: KiOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
        has_control_flow: HasControlFlow,
    ) -> KiRepr {
        let ki_repr = KiRepr::new(
            self.ki_domain_repr,
            opn,
            arguments,
            KiReprSource::Expansion {
                parent_ki_repr: self.parent_ki_repr,
                source,
            },
            self.db,
        );
        if has_control_flow.to_bool() {
            self.ki_domain_repr = KiDomainRepr::ExprNotReturned(ki_repr)
        }
        ki_repr
    }

    pub(crate) fn new_expr_ki_repr(
        &mut self,
        expr: HirLazyExprIdx,
        opn: KiOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
        has_control_flow: HasControlFlow,
    ) -> KiRepr {
        let ki_repr = KiRepr::new(
            self.ki_domain_repr,
            opn,
            arguments,
            KiReprSource::Expansion {
                parent_ki_repr: self.parent_ki_repr,
                source: KiReprExpansionSource::Expr { expr },
            },
            self.db,
        );
        if has_control_flow.to_bool() {
            self.ki_domain_repr = KiDomainRepr::ExprNotReturned(ki_repr)
        }
        ki_repr
    }

    pub(crate) fn new_stmt_ki_repr(
        &mut self,
        stmt: HirLazyStmtIdx,
        opn: KiOpn,
        arguments: SmallVec<[KiArgumentRepr; 4]>,
    ) -> KiRepr {
        let ki_repr = KiRepr::new(
            self.ki_domain_repr,
            opn,
            arguments,
            KiReprSource::Expansion {
                parent_ki_repr: self.parent_ki_repr,
                source: KiReprExpansionSource::Stmt { stmt },
            },
            self.db,
        );
        self.ki_domain_repr = KiDomainRepr::StmtNotReturned(ki_repr);
        ki_repr
    }

    pub(crate) fn after_stmt(&mut self, ki_repr: KiRepr) {
        self.ki_domain_repr = KiDomainRepr::StmtNotReturned(ki_repr)
    }

    // change self
    pub(crate) fn under_condition(&mut self, condition: KiRepr) -> KiDomainReprGuard<'a> {
        self.ki_domain_repr = KiDomainRepr::ConditionNotSatisfied(condition);
        KiDomainReprGuard {
            db: self.db,
            parent_ki_repr: self.parent_ki_repr,
            ki_domain_repr: KiDomainRepr::ConditionSatisfied(condition),
        }
    }
}
