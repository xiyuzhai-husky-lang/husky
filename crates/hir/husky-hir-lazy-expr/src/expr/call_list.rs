use super::*;
use husky_sema_expr::SemaRitchieParameterArgumentMatch;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum HirLazyCallListItemGroup {
    Regular(HirLazyExprIdx),
    Variadic,
    Keyed,
}

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_call_list_item_groups(
        &mut self,
        pams: &[SemaRitchieParameterArgumentMatch],
    ) -> SmallVec<[HirLazyCallListItemGroup; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(
        &mut self,
        pam: &SemaRitchieParameterArgumentMatch,
    ) -> HirLazyCallListItemGroup {
        match pam {
            SemaRitchieParameterArgumentMatch::Regular(_, item) => {
                HirLazyCallListItemGroup::Regular(item.argument_expr_idx().to_hir_lazy(self))
            }
            SemaRitchieParameterArgumentMatch::Variadic(_, _) => HirLazyCallListItemGroup::Variadic,
            SemaRitchieParameterArgumentMatch::Keyed(_, _) => HirLazyCallListItemGroup::Keyed,
        }
    }
}
