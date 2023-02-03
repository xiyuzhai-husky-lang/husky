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
    kind: ImplicitSymbolKind,
    ty: LocalTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitSymbolKind {
    Lifetime(ImplicitLifetimeSymbolKind),
}

impl From<ImplicitLifetimeSymbolKind> for ImplicitSymbolKind {
    fn from(v: ImplicitLifetimeSymbolKind) -> Self {
        Self::Lifetime(v)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitLifetimeSymbolKind {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ImplicitSymbolIdx(usize);

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitSymbolRegistry {
    next: usize,
}

impl ImplicitSymbolRegistry {
    fn next(&mut self) -> ImplicitSymbolIdx {
        let idx = ImplicitSymbolIdx(self.next);
        self.next += 1;
        idx
    }

    fn new_implicit_lifetime_symbol(
        &mut self,
        kind: ImplicitLifetimeSymbolKind,
        term_menu: &TermMenu,
    ) -> ImplicitSymbol {
        self.new_implicit_symbol(kind.into(), term_menu.lifetime_ty().into())
    }

    fn new_implicit_symbol(&mut self, kind: ImplicitSymbolKind, ty: LocalTerm) -> ImplicitSymbol {
        ImplicitSymbol {
            idx: self.next(),
            kind,
            ty,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct UnresolvedTermDurantParameter {
    ty: LocalTerm,
}
