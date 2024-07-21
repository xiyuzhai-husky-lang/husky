use super::*;
use crate::binding::{RustBinding, RustBindings};
use husky_hir_eager_expr::coercion::HirEagerCoercion;
use husky_hir_ty::{
    indirections::{HirIndirection, HirIndirections},
    ritchie::HirRitchieSimpleParameter,
};
use husky_place::place::EthPlace;
use vec_like::{SmallVecMap, SmallVecPairMap};

#[derive(Debug, Default)]
pub(crate) struct HirEagerExprSite {
    pub(crate) rust_precedence_range: RustPrecedenceRange,
    pub(crate) rust_bindings: RustBindings,
}

impl HirEagerExprSite {
    /// generate self subexpr on site
    /// `self` refers to the parent expr on site
    /// this excludes self argument of memoized field, where things are more complicated so that we have to use associated form for calling
    pub(crate) fn simple_self_argument(has_self_value_binding: bool) -> Self {
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

    pub(crate) fn method_self_argument(indirections: &HirIndirections) -> Self {
        let mut rust_precedence_range = RustPrecedenceRange::Geq(RustPrecedence::Suffix);
        let mut rust_bindings: RustBindings = RustBinding::SelfValue.into();
        if indirections.len() > 0 {
            for indirection in &**indirections {
                match indirection {
                    HirIndirection::Place(_) => todo!(),
                    HirIndirection::Deleash => rust_bindings.push(RustBinding::Deleash),
                }
            }
        }
        Self {
            rust_precedence_range,
            rust_bindings,
        }
    }

    /// we have to do more because the contract will not be automatically covered as in previous function
    pub(crate) fn memoized_field_self_argument(
        self_argument_ty: HirType,
        indirections: &HirIndirections,
        db: &::salsa::Db,
    ) -> Self {
        let mut rust_precedence_range = RustPrecedenceRange::Any;
        let mut rust_bindings = RustBindings::default();
        // ad hoc
        if !self_argument_ty.is_always_leashed(db) {
            rust_precedence_range = RustPrecedenceRange::Geq(RustPrecedence::Prefix);
            rust_bindings.push(RustBinding::Releash);
        }
        // ad hoc
        // if indirections.len() > 0 {
        //     todo!();
        // }
        Self {
            rust_precedence_range,
            rust_bindings,
        }
    }

    pub(crate) fn subexpr(rust_precedence_range: RustPrecedenceRange) -> Self {
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
    pub(crate) fn any_precedence() -> Self {
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings: Default::default(),
        }
    }

    pub(crate) fn regular_call_item(
        param: HirRitchieSimpleParameter,
        coercion: HirEagerCoercion,
        db: &::salsa::Db,
    ) -> Self {
        let mut rust_bindings: RustBindings = match param.contract {
            HirContract::Pure => match param.ty.always_copyable(db) {
                true => Default::default(),
                false => RustBinding::Reref.into(),
            },
            HirContract::Move => Default::default(),
            HirContract::Borrow => RustBinding::Reref.into(),
            HirContract::BorrowMut => RustBinding::RerefMut.into(),
            HirContract::Compterm => todo!(),
            HirContract::Leash => todo!(),
            HirContract::At => todo!(),
        };
        match coercion {
            HirEagerCoercion::Trivial(_) => (),
            HirEagerCoercion::Never => (),
            HirEagerCoercion::WrapInSome => rust_bindings.push(RustBinding::WrapInSome),
            HirEagerCoercion::Releash => rust_bindings.push(RustBinding::Reref),
            HirEagerCoercion::Deref(_) => rust_bindings.push(RustBinding::Deref),
        }
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings,
        }
    }

    pub(crate) fn new_root(coercion: Option<HirEagerCoercion>) -> Self {
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings: match coercion {
                Some(coercion) => match coercion {
                    HirEagerCoercion::Trivial(_) => Default::default(),
                    HirEagerCoercion::Never => Default::default(),
                    HirEagerCoercion::WrapInSome => RustBinding::WrapInSome.into(),
                    HirEagerCoercion::Releash => RustBinding::Reref.into(),
                    HirEagerCoercion::Deref(_) => RustBinding::Deref.into(),
                },
                None => Default::default(),
            },
        }
    }

    #[deprecated(note = "change coercion type to HirEagerCoercion")]
    pub(crate) fn new_let_initial_value(
        contract: HirContract,
        initial_value_entry: &HirEagerExprEntry,
        coercion: Option<HirEagerCoercion>,
    ) -> Self {
        let rust_bindings: RustBindings = match initial_value_entry.quary() {
            HirQuary::Transient => Default::default(),
            _ => match contract {
                HirContract::Pure | HirContract::Compterm | HirContract::Leash
                    if !initial_value_entry.is_always_copyable() =>
                {
                    RustBinding::Reref.into()
                }
                HirContract::Borrow => RustBinding::Reref.into(),
                HirContract::BorrowMut => RustBinding::RerefMut.into(),
                _ => Default::default(),
            },
        };
        match coercion {
            Some(_coercion) => (),
            None => (),
        };
        Self {
            rust_precedence_range: RustPrecedenceRange::ANY,
            rust_bindings,
        }
    }
}
