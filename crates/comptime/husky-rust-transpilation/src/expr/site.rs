use super::*;
use crate::binding::{RustBinding, RustBindings};
use husky_hir_eager_expr::{coersion::HirEagerCoersion, HirEagerExprIdx};
use husky_hir_ty::ritchie::HirRitchieRegularParameter;
use husky_opr::precedence::Precedence;
use husky_stack_location::StackLocationIdx;
use husky_term_prelude::TermContract;
use vec_like::SmallVecPairMap;

pub(crate) struct HirEagerExprSite {
    pub(crate) rust_precedence_range: RustPrecedenceRange,
    pub(crate) rust_bindings: RustBindings,
    pub(crate) location_contract_map: SmallVecPairMap<StackLocationIdx, HirEagerContract, 2>,
}

impl HirEagerExprSite {
    /// generate self subexpr on site
    /// `self` refers to the parent expr on site
    pub(crate) fn self_expr_on_site(
        &self,
        self_value_place: HirPlace,
        contract: HirEagerContract,
        has_self_value_binding: bool,
    ) -> Self {
        let mut location_contract_map = self.location_contract_map.clone();
        if let Some(location) = self_value_place.location()
            && contract != HirEagerContract::At
        {
            location_contract_map.insert((location, contract))
        }
        Self {
            location_contract_map,
            rust_precedence_range: RustPrecedenceRange::Geq(RustPrecedence::Suffix),
            // this is because `RustBinding::SelfValue` automatically covers the contract
            rust_bindings: if has_self_value_binding {
                RustBinding::SelfValue.into()
            } else {
                Default::default()
            },
        }
    }

    pub(crate) fn subexpr(&self, rust_precedence_range: RustPrecedenceRange) -> Self {
        Self {
            rust_precedence_range,
            rust_bindings: Default::default(),
            location_contract_map: self.location_contract_map.clone(),
        }
    }

    pub(crate) fn new(rust_precedence_range: RustPrecedenceRange) -> Self {
        Self {
            rust_precedence_range,
            rust_bindings: Default::default(),
            location_contract_map: Default::default(),
        }
    }

    #[track_caller]
    pub(crate) fn any_precedence(&self) -> Self {
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings: Default::default(),
            location_contract_map: self.location_contract_map.clone(),
        }
    }

    pub(crate) fn regular_call_item(
        &self,
        param: HirRitchieRegularParameter,
        coersion: HirEagerCoersion,
        db: &::salsa::Db,
    ) -> Self {
        let mut rust_bindings: RustBindings = match param.contract {
            HirEagerContract::Pure => match param.ty.always_copyable(db) {
                true => Default::default(),
                false => RustBinding::Reref.into(),
            },
            HirEagerContract::Move => Default::default(),
            HirEagerContract::Borrow => RustBinding::Reref.into(),
            HirEagerContract::BorrowMut => RustBinding::RerefMut.into(),
            HirEagerContract::Const => todo!(),
            HirEagerContract::Leash => todo!(),
            HirEagerContract::At => todo!(),
        };
        match coersion {
            HirEagerCoersion::Trivial(_) => (),
            HirEagerCoersion::Never => (),
            HirEagerCoersion::WrapInSome => rust_bindings.push(RustBinding::WrapInSome),
            HirEagerCoersion::PlaceToLeash => rust_bindings.push(RustBinding::Reref),
            HirEagerCoersion::Deref(_) => rust_bindings.push(RustBinding::Deref),
        }
        let mut location_contract_map = self.location_contract_map.clone();
        if let Some(location) = coersion.place_after_coersion().location() {
            location_contract_map.insert((location, param.contract))
        }
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings,
            location_contract_map,
        }
    }

    #[deprecated(note = "change coersion type to HirEagerCoersion")]
    pub(crate) fn new_root(coersion: Option<HirEagerCoersion>) -> Self {
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings: match coersion {
                Some(coersion) => match coersion {
                    HirEagerCoersion::Trivial(_) => Default::default(),
                    HirEagerCoersion::Never => Default::default(),
                    HirEagerCoersion::WrapInSome => RustBinding::WrapInSome.into(),
                    HirEagerCoersion::PlaceToLeash => RustBinding::Reref.into(),
                    HirEagerCoersion::Deref(_) => RustBinding::Deref.into(),
                },
                None => Default::default(),
            },
            location_contract_map: Default::default(),
        }
    }

    pub(crate) fn location_contract(&self, location: StackLocationIdx) -> Option<HirEagerContract> {
        self.location_contract_map.get_value(location).copied()
    }
}
