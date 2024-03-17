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
    Trait,
    ReturnType,
    PropsStructFieldType { ident_token: IdentRegionalToken },
    TupleStructFieldType,
    BlockExpr,
    ReturnExpr,
    Condition,
    ParenateParameterDefaultValue { ty_syn_expr_idx: SynExprIdx },
    FieldBindInitialValue { ty_syn_expr_idx: SynExprIdx },
    ConstantImplicitParameterType,
    ExplicitParameterType,
    HtmlArgumentExpr,
    LetStmtType,
    LetStmtInitialValue,
    Snippet,
    ValExpr,
    EvalExpr,
    AssocTypeTerm,
    TypeAliasTypeTerm,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynPatternRoot {
    kind: SynPatternExprRootKind,
    syn_pattern_expr_idx: SynPatternIdx,
}

impl From<ParenateParameterSynPatternExprRoot> for SynPatternRoot {
    fn from(root: ParenateParameterSynPatternExprRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternExprRootKind::Parenate,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<ClosureSynPatternExprRoot> for SynPatternRoot {
    fn from(root: ClosureSynPatternExprRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternExprRootKind::Parenate,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<LetPatternSynExprRoot> for SynPatternRoot {
    fn from(root: LetPatternSynExprRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternExprRootKind::Let,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<BeSynPatternExprRoot> for SynPatternRoot {
    fn from(root: BeSynPatternExprRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternExprRootKind::Be,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<CaseSynPatternExprRoot> for SynPatternRoot {
    fn from(root: CaseSynPatternExprRoot) -> Self {
        SynPatternRoot {
            kind: SynPatternExprRootKind::Case,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SynPatternExprRootKind {
    Parenate,
    Let,
    Case,
    Be,
    Closure,
}

impl SynPatternRoot {
    pub(crate) fn new(
        kind: SynPatternExprRootKind,
        syn_pattern_expr_idx: SynPatternIdx,
        ctx: &mut SynExprContext,
    ) -> Self {
        let slf = Self {
            kind,
            syn_pattern_expr_idx,
        };
        ctx.add_pattern_expr_root(slf);
        slf
    }

    pub fn syn_pattern_expr_idx(self) -> SynPatternIdx {
        self.syn_pattern_expr_idx
    }

    pub fn kind(&self) -> SynPatternExprRootKind {
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
