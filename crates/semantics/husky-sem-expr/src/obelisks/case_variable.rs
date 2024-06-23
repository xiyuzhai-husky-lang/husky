use super::*;
use husky_regional_token::ColonRegionalToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaseVariableObelisk {
    syn_pattern_root: CaseSynPatternRoot,
    variables: CurrentVariableIdxRange,
}

impl CaseVariableObelisk {
    pub fn syn_pattern_root(&self) -> CaseSynPatternRoot {
        self.syn_pattern_root
    }

    pub fn variables(&self) -> CurrentVariableIdxRange {
        self.variables
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_case_pattern_sem_obelisk(
        &mut self,
        case_pattern_syn_obelisk: &CasePatternSyndicate,
        match_target_ty: FlyTerm,
    ) -> CaseVariableObelisk {
        self.infer_variable_pattern_root_and_symbols_ty(
            case_pattern_syn_obelisk.syn_pattern_root(),
            match_target_ty,
            case_pattern_syn_obelisk.variables(),
        );
        CaseVariableObelisk {
            syn_pattern_root: case_pattern_syn_obelisk.syn_pattern_root(),
            variables: case_pattern_syn_obelisk.variables(),
        }
    }
}
