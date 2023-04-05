use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ImplicitSymbolIdx(usize);

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ImplicitSymbolRegistry {
    next: usize,
}

impl ImplicitSymbolRegistry {
    fn next(&mut self) -> ImplicitSymbolIdx {
        let idx = ImplicitSymbolIdx(self.next);
        self.next += 1;
        idx
    }

    fn new_implicit_symbol(
        &mut self,
        src_expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> ImplicitSymbol {
        ImplicitSymbol {
            idx: self.next(),
            src_expr_idx,
            variant,
        }
    }
}

impl UnresolvedTerms {
    pub(crate) fn new_implicit_symbol(
        &mut self,
        src_expr_idx: ExprIdx,
        variant: ImplicitSymbolVariant,
    ) -> UnresolvedTermIdx {
        let new_implicit_symbol = self
            .implicit_symbol_registry
            .new_implicit_symbol(src_expr_idx, variant);
        self.alloc_unresolved_term(
            src_expr_idx,
            LocalTermData::ImplicitSymbol(new_implicit_symbol),
        )
    }

    pub(crate) fn new_implicit_symbol_from_parameter_symbol(
        &mut self,
        db: &dyn TermDb,
        src_expr_idx: ExprIdx,
        parameter_symbol: LocalTerm,
    ) -> UnresolvedTermIdx {
        let variant = match parameter_symbol.pattern_inner(db, self) {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                path,
                refined_path,
                argument_tys: arguments,
            } => todo!(),
            LocalTermPattern::Curry {
                curry_kind,
                variance,
                parameter_variable: parameter_symbol,
                parameter_ty,
                return_ty,
            } => todo!(),
            LocalTermPattern::Ritchie { .. } => todo!(),
            LocalTermPattern::ImplicitSymbol(_, _) => todo!(),
            LocalTermPattern::Category(_) => todo!(),
        };
        self.new_implicit_symbol(src_expr_idx, variant)
    }
}
