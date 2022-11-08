use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ConstExprPatternItd {
    term: TermPatternItd,
    opt_substitution_ctx_idx: Option<TermSubstitutionContextIdx>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ConstExprPattern {
    term: TermPatternItd,
    opt_substitution_ctx: Option<TermSubstitutionContext>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TermSubstitutionContextIdx(usize);

#[derive(Debug, PartialEq, Eq)]
pub struct TermSubstitutionContext {}
