use super::*;
use crate::binding::{RustBinding, RustBindings};
use husky_hir_eager_expr::coercion::{
    DedirectionHirEagerCoercion, HirEagerCoercion, RedirectionHirEagerCoercion,
};
use husky_hir_ty::{
    indirections::{HirIndirection, HirIndirections},
    ritchie::HirRitchieSimpleParameter,
};
use husky_place::place::EthPlace;
use vec_like::{SmallVecMap, SmallVecPairMap};

#[derive(Debug, Clone, Copy)]
pub(crate) enum HirEagerExprRole<'db> {
    SimpleSelfArgument,
    AssignSelfArgument,
    SelfArgumentWithIndirection {
        indirections: &'db HirIndirections,
    },
    MemoizedFieldSelfArgument {
        indirections: &'db HirIndirections,
    },
    Subexpr {
        outermost_precedence_range: RustPrecedenceRange,
    },
    RegularCallItem,
    Root,
    LetInitialValue,
}

impl<'db> HirEagerExprRole<'db> {
    /// generate self subexpr on site
    /// `self` refers to the parent expr on site
    /// this excludes self argument of memoized field, where things are more complicated so that we have to use associated form for calling
    pub(crate) fn simple_self_argument() -> Self {
        HirEagerExprRole::SimpleSelfArgument
    }

    pub(crate) fn assign_self_argument() -> Self {
        HirEagerExprRole::AssignSelfArgument
    }

    pub(crate) fn self_argument_with_indirections(indirections: &'db HirIndirections) -> Self {
        HirEagerExprRole::SelfArgumentWithIndirection { indirections }
    }

    /// we have to do more because the contract will not be automatically covered as in previous function
    pub(crate) fn memoized_field_self_argument(
        self_argument_ty: HirType,
        indirections: &'db HirIndirections,
        db: &'db ::salsa::Db,
    ) -> Self {
        HirEagerExprRole::MemoizedFieldSelfArgument { indirections }
    }

    pub(crate) fn subexpr(outermost_precedence_range: RustPrecedenceRange) -> Self {
        HirEagerExprRole::Subexpr {
            outermost_precedence_range,
        }
    }

    #[track_caller]
    pub(crate) fn subexpr_with_any_precedence_range() -> Self {
        Self::subexpr(RustPrecedenceRange::ANY)
    }

    pub(crate) fn regular_call_item() -> Self {
        HirEagerExprRole::RegularCallItem
    }

    pub(crate) fn new_root() -> Self {
        HirEagerExprRole::Root
    }

    #[deprecated(note = "change coercion type to HirEagerCoercion")]
    pub(crate) fn new_let_initial_value() -> Self {
        HirEagerExprRole::LetInitialValue
    }

    pub(crate) fn html_argument() -> Self {
        HirEagerExprRole::RegularCallItem
    }
}
