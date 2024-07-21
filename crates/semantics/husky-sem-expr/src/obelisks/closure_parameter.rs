use super::*;
use husky_regional_token::ColonRegionalToken;
use husky_syn_expr::syndicates::closure_parameter::ClosureParameterSyndicate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureParameterObelisk {
    Simple {
        syn_pattern_root: ClosureSynPatternRoot,
        variables: CurrentVariableIdxRange,
        colon_token: Option<ColonRegionalToken>,
        ty: Option<SemExprIdx>,
    },
}

impl ClosureParameterObelisk {
    pub fn variables(self) -> CurrentVariableIdxRange {
        match self {
            ClosureParameterObelisk::Simple { variables, .. } => variables,
        }
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_closure_parameter_obelisk(
        &mut self,
        closure_pattern_syndicate: &ClosureParameterSyndicate,
    ) -> ClosureParameterObelisk {
        match *closure_pattern_syndicate {
            ClosureParameterSyndicate::Simple {
                syn_pattern_root,
                variables,
                ty,
            } => ClosureParameterObelisk::Simple {
                syn_pattern_root,
                variables,
                colon_token: ty.map(|(colon_token, _)| colon_token),
                ty: ty.map(|(_, ty)| self.build_expr(ty, ExpectSort::TYPE)),
            },
        }
    }
}
