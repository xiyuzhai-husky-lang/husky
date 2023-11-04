use husky_regional_token::ColonRegionalToken;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CasePatternSemaObelisk {
    syn_pattern_root: SynPatternRoot,
    variables: SynCurrentSymbolIdxRange,
}

impl CasePatternSemaObelisk {
    pub fn syn_pattern_root(&self) -> SynPatternRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> SynCurrentSymbolIdxRange {
        self.variables
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_case_pattern_sema_obelisk(
        &mut self,
        case_pattern_syn_obelisk: &CasePatternObelisk,
        match_target_ty: FluffyTerm,
    ) -> CasePatternSemaObelisk {
        self.infer_pattern_root_and_symbols_ty(
            case_pattern_syn_obelisk.syn_pattern_root(),
            match_target_ty,
            case_pattern_syn_obelisk.variables(),
        );
        CasePatternSemaObelisk {
            syn_pattern_root: case_pattern_syn_obelisk.syn_pattern_root(),
            variables: case_pattern_syn_obelisk.variables(),
        }
    }
}
