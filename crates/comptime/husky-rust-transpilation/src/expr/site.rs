use super::{coersion::HirCoersion, *};
use crate::binding::{RustBinding, RustBindings};
use husky_hir_eager_expr::HirEagerExprIdx;
use husky_opr::precedence::Precedence;
use husky_stack_location::StackLocationIdx;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

pub(crate) struct HirExprSite {
    outer_rust_precedence_range: RustPrecedenceRange,
    coersion: HirCoersion,
    outer_rust_bindings: RustBindings,
    location_contract_map: SmallVecPairMap<StackLocationIdx, Contract, 2>,
}

impl HirExprSite {
    /// generate self site for subexpr
    /// `self` refers to the parent site
    pub(crate) fn self_expr_site(&self, coersion: HirCoersion) -> Self {
        let mut location_contract_map = self.location_contract_map.clone();
        match coersion {
            _ => todo!("add location contract key value pair"),
        }
        Self {
            outer_rust_precedence_range: RustPrecedenceRange::Geq(RustPrecedence::Suffix),
            coersion,
            outer_rust_bindings: RustBinding::SelfValue.into(),
            location_contract_map,
        }
    }
}

pub(crate) struct HirEagerExprOnSite {
    hir_eager_expr_idx: HirEagerExprIdx,
    site: HirExprSite,
}

impl HirEagerExprOnSite {
    /// generate self subexpr on site
    /// `self` refers to the parent expr on site
    pub(crate) fn self_expr_on_site(
        &self,
        hir_eager_expr_idx: HirEagerExprIdx,
        coersion: HirCoersion,
    ) -> Self {
        Self {
            hir_eager_expr_idx,
            site: self.site.self_expr_site(coersion),
        }
    }
}
