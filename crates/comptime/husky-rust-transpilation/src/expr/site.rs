use super::*;
use crate::binding::{RustBinding, RustBindings};
use husky_hir_eager_expr::{coersion::HirEagerCoersion, HirEagerExprIdx};
use husky_opr::precedence::Precedence;
use husky_stack_location::StackLocationIdx;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

pub(crate) struct HirEagerExprSite {
    pub(crate) outer_rust_precedence_range: RustPrecedenceRange,
    pub(crate) outer_rust_bindings: RustBindings,
    location_contract_map: SmallVecPairMap<StackLocationIdx, Contract, 2>,
}

impl HirEagerExprSite {
    /// generate self subexpr on site
    /// `self` refers to the parent expr on site
    pub(crate) fn self_expr_on_site(&self, coersion: HirEagerCoersion, contract: Contract) -> Self {
        let mut location_contract_map = self.location_contract_map.clone();
        if let Some(location) = coersion.place_after_coersion().location() {
            location_contract_map.insert((location, contract))
        }
        Self {
            location_contract_map,
            outer_rust_precedence_range: RustPrecedenceRange::Geq(RustPrecedence::Suffix),
            // this is because `RustBinding::SelfValue` automatically covers the contract
            outer_rust_bindings: RustBinding::SelfValue.into(),
        }
    }

    pub(crate) fn location_contract(&self, location: StackLocationIdx) -> Option<Contract> {
        self.location_contract_map.get_value(location).copied()
    }
}
