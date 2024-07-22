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
    LeashlessSelfArgument {
        indirections: &'db HirIndirections,
    },
    LeashedSelfArgument {
        indirections: &'db HirIndirections,
    },
    Subexpr {
        outermost_precedence_range: RustPrecedenceRange,
    },
    RegularCallItem {
        contract: HirContract,
    },
    Root,
    PatternOpd {
        contract: HirContract,
    },
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

    pub(crate) fn leashless_self_argument(indirections: &'db HirIndirections) -> Self {
        HirEagerExprRole::LeashlessSelfArgument { indirections }
    }

    /// we have to do more because the contract will not be automatically covered as in previous function
    pub(crate) fn leashed_self_argument(
        indirections: &'db HirIndirections,
        db: &'db ::salsa::Db,
    ) -> Self {
        HirEagerExprRole::LeashedSelfArgument { indirections }
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

    pub(crate) fn regular_call_item(contract: HirContract) -> Self {
        HirEagerExprRole::RegularCallItem { contract }
    }

    pub(crate) fn new_root() -> Self {
        HirEagerExprRole::Root
    }

    pub(crate) fn new_pattern_opd(contract: HirContract) -> Self {
        HirEagerExprRole::PatternOpd { contract }
    }

    pub(crate) fn html_argument() -> Self {
        HirEagerExprRole::RegularCallItem {
            contract: HirContract::Pure,
        }
    }
}
