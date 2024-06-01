use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynExprRoot {
    kind: SynExprRootKind,
    syn_expr_idx: SynExprIdx,
}

impl SynExprRoot {
    fn new(kind: SynExprRootKind, syn_expr_idx: SynExprIdx) -> Self {
        Self { kind, syn_expr_idx }
    }

    pub fn kind(&self) -> SynExprRootKind {
        self.kind
    }

    pub fn syn_expr_idx(&self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SynExprRootKind {
    SelfType,
    PrimalTrait,
    ReturnType,
    PropsStructFieldType {
        ident_token: IdentRegionalToken,
    },
    TupleStructFieldType,
    BlockExpr,
    ReturnExpr,
    Condition,
    ParenateParameterDefaultValue {
        ty_syn_expr_idx: SynExprIdx,
    },
    FieldBindInitialValue {
        ty_syn_expr_idx: SynExprIdx,
    },
    ConstantImplicitParameterType,
    ExplicitParameterType,
    HtmlArgumentExpr,
    LetStmtType,
    LetStmtInitialValue,
    Snippet,
    ValExpr,
    StaticExpr,
    EvalExpr,
    AssocTypeTerm,
    TypeAliasTypeTerm,
    /// these are traits in the trait constraints
    TraitInConstraint,
    Effect,
    DefaultConstExclude,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynPatternRoot {
    kind: SynPatternRootKind,
    syn_pattern_idx: SynPatternIdx,
}

impl From<ParenateParameterSynPatternRoot> for SynPatternRoot {
    fn from(root: ParenateParameterSynPatternRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternRootKind::Parenate,
            syn_pattern_idx: root.syn_pattern_idx(),
        }
    }
}

impl From<ClosureSynPatternRoot> for SynPatternRoot {
    fn from(root: ClosureSynPatternRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternRootKind::Parenate,
            syn_pattern_idx: root.syn_pattern_idx(),
        }
    }
}

impl From<LetPatternSynExprRoot> for SynPatternRoot {
    fn from(root: LetPatternSynExprRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternRootKind::Let,
            syn_pattern_idx: root.syn_pattern_idx(),
        }
    }
}

impl From<BeSynPatternRoot> for SynPatternRoot {
    fn from(root: BeSynPatternRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternRootKind::Be,
            syn_pattern_idx: root.syn_pattern_idx(),
        }
    }
}

impl From<CaseSynPatternRoot> for SynPatternRoot {
    fn from(root: CaseSynPatternRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternRootKind::Case,
            syn_pattern_idx: root.syn_pattern_idx(),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SynPatternRootKind {
    Parenate,
    Let,
    Case,
    Be,
    Closure,
}

impl SynPatternRoot {
    pub(crate) fn new(
        kind: SynPatternRootKind,
        syn_pattern_idx: SynPatternIdx,
        ctx: &mut SynExprContext,
    ) -> Self {
        let slf = Self {
            kind,
            syn_pattern_idx,
        };
        ctx.add_pattern_expr_root(slf);
        slf
    }

    pub fn syn_pattern_idx(self) -> SynPatternIdx {
        self.syn_pattern_idx
    }

    pub fn kind(&self) -> SynPatternRootKind {
        self.kind
    }
}

impl<'a> SynExprContext<'a> {
    pub(crate) fn add_expr_root(&mut self, kind: SynExprRootKind, expr: SynExprIdx) {
        self.syn_expr_roots.push(SynExprRoot::new(kind, expr))
    }

    // move to somewhere else?
    pub(crate) fn add_pattern_expr_root(&mut self, syn_pattern_expr_root: SynPatternRoot) {
        self.syn_pattern_expr_roots.push(syn_pattern_expr_root)
    }
}
