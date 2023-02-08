use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UnresolvedTerm {
    ImplicitSymbol(ImplicitSymbol),
    Curry {
        // todo: include parameter
        // so that the following dependent type is possible:
        // (u: Universe) -> (a: Type u + 1) -> List a -> a
        parameter_ty: LocalTerm,
        return_ty: LocalTerm,
    },
    Application {
        function: LocalTerm,
        argument: LocalTerm,
    },
    Abstraction {
        parameter: TermSymbol,
        body: LocalTerm,
    },
    Durant {
        durant_kind: TermDurantKind,
        parameter_book_tys: Vec<UnresolvedTermDurantParameterBookType>,
        return_ty: LocalTerm,
    },
    Subentity {},
    AsTraitSubentity {},
    TraitConstraint {},
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ImplicitSymbol {
    idx: ImplicitSymbolIdx,
    src_expr_idx: ExprIdx,
    variant: ImplicitSymbolVariant,
}

impl ImplicitSymbol {
    pub(crate) fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn variant(&self) -> &ImplicitSymbolVariant {
        &self.variant
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImplicitSymbolVariant {
    Lifetime,
    UnspecifiedIntegerType,
    UnspecifiedFloatType,
    ImplicitType,
}

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

    pub(super) fn new_implicit_symbol(
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermDurantParameterBookType {
    ty: LocalTerm,
}
