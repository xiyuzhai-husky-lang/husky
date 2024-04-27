use super::*;
use husky_regional_token::ColonRegionalToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LetVariableObelisk {
    syn_pattern_root: LetPatternSynExprRoot,
    variables: CurrentSynSymbolIdxRange,
    colon_token: Option<ColonRegionalToken>,
    ty_sem_expr_idx: Option<SemExprIdx>,
}

impl LetVariableObelisk {
    pub fn syn_pattern_root(&self) -> LetPatternSynExprRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> CurrentSynSymbolIdxRange {
        self.variables
    }

    pub fn colon_token(&self) -> Option<ColonRegionalToken> {
        self.colon_token
    }

    pub fn ty_sem_expr_idx(&self) -> Option<SemExprIdx> {
        self.ty_sem_expr_idx
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_let_pattern_obelisk(
        &mut self,
        let_pattern_syn_obelisk: &'a LetPatternSyndicate,
    ) -> SynExprResultRef<'a, LetVariableObelisk> {
        Ok(LetVariableObelisk {
            syn_pattern_root: let_pattern_syn_obelisk.syn_pattern_root(),
            variables: let_pattern_syn_obelisk.variables(),
            colon_token: let_pattern_syn_obelisk.colon_token()?,
            ty_sem_expr_idx: let_pattern_syn_obelisk
                .ty_syn_expr_idx()
                .map(|ty_syn_expr_idx| self.build_sem_expr(ty_syn_expr_idx, ExpectSort::TYPE)),
        })
    }
}
