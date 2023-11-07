use husky_regional_token::ColonRegionalToken;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetPatternSemaSyndicate {
    syn_pattern_root: LetSynPatternExprRoot,
    variables: CurrentSynSymbolIdxRange,
    colon_token: Option<ColonRegionalToken>,
    ty_sema_expr_idx: Option<SemaExprIdx>,
}

impl LetPatternSemaSyndicate {
    pub fn syn_pattern_root(&self) -> LetSynPatternExprRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }

    pub fn colon_token(&self) -> Option<ColonRegionalToken> {
        self.colon_token
    }

    pub fn ty_sema_expr_idx(&self) -> Option<SemaExprIdx> {
        self.ty_sema_expr_idx
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_let_pattern_sema_obelisk(
        &mut self,
        let_pattern_syn_obelisk: &'a LetPatternSynSyndicate,
    ) -> SynExprResultRef<'a, LetPatternSemaSyndicate> {
        Ok(LetPatternSemaSyndicate {
            syn_pattern_root: let_pattern_syn_obelisk.syn_pattern_root(),
            variables: let_pattern_syn_obelisk.variables(),
            colon_token: let_pattern_syn_obelisk.colon_token()?,
            ty_sema_expr_idx: let_pattern_syn_obelisk
                .ty_syn_expr_idx()
                .map(|ty_syn_expr_idx| {
                    self.build_sema_expr(
                        ty_syn_expr_idx,
                        ExpectEqsCategory::new_expect_eqs_ty_kind(),
                    )
                }),
        })
    }
}
