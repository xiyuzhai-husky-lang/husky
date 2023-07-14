use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ExprRoot {
    kind: ExprRootKind,
    expr_idx: ExprIdx,
}

impl ExprRoot {
    fn new(kind: ExprRootKind, expr_idx: ExprIdx) -> Self {
        Self { kind, expr_idx }
    }

    pub fn kind(&self) -> ExprRootKind {
        self.kind
    }

    pub fn expr_idx(&self) -> ExprIdx {
        self.expr_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ExprRootKind {
    SelfType,
    Trait,
    ReturnType,
    PropsStructFieldType { ident_token: IdentToken },
    TupleStructFieldType,
    BlockExpr,
    ReturnExpr,
    Condition,
    ExplicitParameterDefaultValue { ty_expr_idx: ExprIdx },
    FieldBindInitialValue { ty_expr_idx: ExprIdx },
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

impl<'a> ExprParser<'a> {
    pub(super) fn add_expr_root(&mut self, kind: ExprRootKind, expr: ExprIdx) {
        self.expr_roots.push(ExprRoot::new(kind, expr))
    }
}
