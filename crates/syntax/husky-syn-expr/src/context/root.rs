use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynExprRootKind {
    SelfType,
    Trait,
    ReturnType,
    PropsStructFieldType { ident_token: IdentRegionalToken },
    TupleStructFieldType,
    BlockExpr,
    ReturnExpr,
    Condition,
    ExplicitParameterDefaultValue { ty_syn_expr_idx: SynExprIdx },
    FieldBindInitialValue { ty_syn_expr_idx: SynExprIdx },
    ConstantImplicitParameterType,
    ExplicitParameterType,
    HtmlArgumentExpr,
    LetStmtType,
    LetStmtInitialValue,
    Snippet,
    Traits,
    ValExpr,
    EvalExpr,
    AssociatedTypeTerm,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub struct SynPatternExprRoot {
    kind: SynPatternExprRootKind,
    syn_pattern_expr_idx: SynPatternExprIdx,
}

impl From<ParenateSynPatternExprRoot> for SynPatternExprRoot {
    fn from(root: ParenateSynPatternExprRoot) -> Self {
        SynPatternExprRoot {
            kind: SynPatternExprRootKind::Parenate,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<LetSynPatternExprRoot> for SynPatternExprRoot {
    fn from(root: LetSynPatternExprRoot) -> Self {
        SynPatternExprRoot {
            kind: SynPatternExprRootKind::Let,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<BeSynPatternExprRoot> for SynPatternExprRoot {
    fn from(root: BeSynPatternExprRoot) -> Self {
        SynPatternExprRoot {
            kind: SynPatternExprRootKind::Be,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

impl From<CaseSynPatternExprRoot> for SynPatternExprRoot {
    fn from(root: CaseSynPatternExprRoot) -> Self {
        SynPatternExprRoot {
            kind: SynPatternExprRootKind::Case,
            syn_pattern_expr_idx: root.syn_pattern_expr_idx(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynPatternExprRootKind {
    Parenate,
    Let,
    Case,
    Be,
}

impl SynPatternExprRoot {
    pub(crate) fn new(
        kind: SynPatternExprRootKind,
        syn_pattern_expr_idx: SynPatternExprIdx,
        ctx: &mut SynExprContext,
    ) -> Self {
        let slf = Self {
            kind,
            syn_pattern_expr_idx,
        };
        ctx.add_pattern_expr_root(slf);
        slf
    }

    pub fn syn_pattern_expr_idx(self) -> SynPatternExprIdx {
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
    pub(crate) fn add_pattern_expr_root(&mut self, syn_pattern_expr_root: SynPatternExprRoot) {
        self.syn_pattern_expr_roots.push(syn_pattern_expr_root)
    }
}
