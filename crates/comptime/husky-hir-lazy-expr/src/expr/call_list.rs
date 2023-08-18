use super::*;
use husky_expr_ty::RitchieParameterArgumentMatch;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum HirLazyCallListItemGroup {
    Regular(HirLazyExprIdx),
    Variadic,
    Keyed,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_call_list_item_groups(
        &mut self,
        pams: &[RitchieParameterArgumentMatch],
    ) -> SmallVec<[HirLazyCallListItemGroup; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(
        &mut self,
        pam: &RitchieParameterArgumentMatch,
    ) -> HirLazyCallListItemGroup {
        match pam {
            RitchieParameterArgumentMatch::Regular(_, item) => {
                HirLazyCallListItemGroup::Regular(item.argument_expr_idx().to_hir_lazy(self))
            }
            RitchieParameterArgumentMatch::Variadic(_, _) => HirLazyCallListItemGroup::Variadic,
            RitchieParameterArgumentMatch::Keyed(_, _) => HirLazyCallListItemGroup::Keyed,
        }
    }
}
