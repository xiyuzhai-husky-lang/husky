use husky_expr_syntax::RawExprIdx;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct UnresolvedTermIdx(usize);

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnresolvedTermRegistry {
    terms: Vec<UnresolvedTerm>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnresolvedTerm {
    Implicit,
    IntegerLiteral(RawExprIdx),
    IntegerType(UnresolvedTermIdx),
    FloatLiteral(RawExprIdx),
    FloatType(UnresolvedTermIdx),
}

impl UnresolvedTermRegistry {
    pub(crate) fn issue(&mut self, term: UnresolvedTerm) -> UnresolvedTermIdx {
        let raw = self.terms.len();
        self.terms.push(term);
        UnresolvedTermIdx(raw)
    }
}
