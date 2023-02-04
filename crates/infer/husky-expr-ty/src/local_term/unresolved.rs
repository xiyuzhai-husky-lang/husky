use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum UnresolvedTerm {
    ImplicitSymbol(ImplicitSymbol),
    Curry {
        x: LocalTerm,
        y: LocalTerm,
    },
    Application {
        m: LocalTerm,
        n: LocalTerm,
    },
    Abstraction {
        x: TermSymbol,
        m: LocalTerm,
    },
    Durant {
        kind: TermDurantKind,
        params: Vec<UnresolvedTermDurantParameter>,
        y: LocalTerm,
    },
    Subentity {},
    AsTraitSubentity {},
    TraitConstraint {},
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitSymbol {
    idx: ImplicitSymbolIdx,
    src_expr_idx: ExprIdx,
    variant: ImplicitSymbolVariant,
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
pub(crate) struct UnresolvedTermDurantParameter {
    ty: LocalTerm,
}
