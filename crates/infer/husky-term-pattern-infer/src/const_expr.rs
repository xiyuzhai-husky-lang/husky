use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ConstExprPatternItd {
    term: TermPatternItd,
    opt_substitution_ctx_idx: Option<TermSubstitutionContextIdx>,
}

impl ConstExprPatternItd {
    pub(crate) fn new(patt: ConstExprPattern) -> Self {
        if let Some(_) = patt.opt_substitution_ctx {
            todo!()
        }
        Self {
            term: patt.term,
            opt_substitution_ctx_idx: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ConstExprPattern {
    pub(crate) term: TermPatternItd,
    pub(crate) opt_substitution_ctx: Option<TermSubstitutionContext>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TermSubstitutionContextIdx(usize);

#[derive(Debug, PartialEq, Eq)]
pub struct TermSubstitutionContext {}
