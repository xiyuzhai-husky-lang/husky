use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynExprRoot {
    kind: ExprRootKind,
    syn_expr_idx: SynExprIdx,
}

impl SynExprRoot {
    fn new(kind: ExprRootKind, syn_expr_idx: SynExprIdx) -> Self {
        Self { kind, syn_expr_idx }
    }

    pub fn kind(&self) -> ExprRootKind {
        self.kind
    }

    pub fn syn_expr_idx(&self) -> SynExprIdx {
        self.syn_expr_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExprRootKind {
    SelfType,
    Trait,
    ReturnType,
    PropsStructFieldType { ident_token: IdentRegionalToken },
    TupleStructFieldType,
    BlockExpr,
    ReturnExpr,
    Condition,
    ExplicitParameterDefaultValue { ty_expr_idx: SynExprIdx },
    FieldBindInitialValue { ty_expr_idx: SynExprIdx },
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

impl<'a> SynExprContext<'a> {
    pub(crate) fn add_expr_root(&mut self, kind: ExprRootKind, expr: SynExprIdx) {
        self.syn_expr_roots.push(SynExprRoot::new(kind, expr))
    }
}
