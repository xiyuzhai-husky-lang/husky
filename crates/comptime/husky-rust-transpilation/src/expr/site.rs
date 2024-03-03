use super::*;
use crate::binding::{RustBinding, RustBindings};
use husky_hir_eager_expr::coersion::HirEagerCoersion;
use husky_hir_ty::ritchie::HirRitchieSimpleParameter;
use husky_place::place::Place;
use vec_like::{SmallVecMap, SmallVecPairMap};

#[derive(Debug, Default)]
pub(crate) struct HirEagerExprSite {
    pub(crate) rust_precedence_range: RustPrecedenceRange,
    pub(crate) rust_bindings: RustBindings,
}

impl HirEagerExprSite {
    /// generate self subexpr on site
    /// `self` refers to the parent expr on site
    pub(crate) fn self_expr_on_site(&self, has_self_value_binding: bool) -> Self {
        Self {
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
        }
    }

    pub(crate) fn new(rust_precedence_range: RustPrecedenceRange) -> Self {
        Self {
            rust_precedence_range,
            rust_bindings: Default::default(),
        }
    }

    #[track_caller]
    pub(crate) fn any_precedence(&self) -> Self {
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings: Default::default(),
        }
    }

    pub(crate) fn regular_call_item(
        &self,
        param: HirRitchieSimpleParameter,
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
        let mut place_contracts: SmallVecPairMap<Place, HirEagerContract, 2> = Default::default();
        if let Some(place) = coersion.quary_after_coersion().place() {
            place_contracts.insert((place, param.contract))
        }
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings,
        }
    }

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
        }
    }

    #[deprecated(note = "change coersion type to HirEagerCoersion")]
    pub(crate) fn new_let_initial_value(
        contract: HirEagerContract,
        initial_value_entry: &HirEagerExprEntry,
        coersion: Option<HirEagerCoersion>,
    ) -> Self {
        let mut place_contracts: SmallVecPairMap<Place, HirEagerContract, 2> = Default::default();
        if let Some(place) = initial_value_entry.quary().place()
            && contract != HirEagerContract::At
        {
            place_contracts.insert((place, contract))
        };
        let rust_bindings: RustBindings = match initial_value_entry.quary() {
            HirQuary::Transient => Default::default(),
            _ => match contract {
                HirEagerContract::Pure | HirEagerContract::Const | HirEagerContract::Leash
                    if !initial_value_entry.is_always_copyable() =>
                {
                    RustBinding::Reref.into()
                }
                HirEagerContract::Borrow => RustBinding::Reref.into(),
                HirEagerContract::BorrowMut => RustBinding::RerefMut.into(),
                _ => Default::default(),
            },
        };
        match coersion {
            Some(_coersion) => (),
            None => (),
        };
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings,
        }
    }
}
