use super::*;
use husky_hir_ty::ritchie::HirEagerContract;
use husky_syn_expr::SelfValueParameterSyndicate;
use husky_term_prelude::TermContract;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirEagerSelfValueParameter {
    pub contract: HirEagerContract,
    pub self_ty: HirType,
}

impl HirEagerSelfValueParameter {
    pub(crate) fn from_syn(
        self_ty: HirType,
        syndicate: Option<SelfValueParameterSyndicate>,
    ) -> Self {
        HirEagerSelfValueParameter {
            self_ty,
            contract: HirEagerContract::from_term(TermContract::new(
                syndicate
                    .map(|syndicate| syndicate.ephem_symbol_modifier_token_group())
                    .flatten(),
            )),
        }
    }
}
