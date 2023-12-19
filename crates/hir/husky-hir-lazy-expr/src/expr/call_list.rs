use super::*;
use husky_sema_expr::SemaRitchieParameterArgumentMatch;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[enum_class::from_variants]
pub enum HirLazyCallListItemGroup {
    Regular(HirLazyExprIdx),
    Variadic(Vec<HirLazyExprIdx>),
    Keyed(Ident, Option<HirLazyExprIdx>),
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
                HirLazyCallListItemGroup::Regular(item.argument_sema_expr_idx().to_hir_lazy(self))
            }
            SemaRitchieParameterArgumentMatch::Variadic(_, items) => {
                HirLazyCallListItemGroup::Variadic(
                    items
                        .iter()
                        .map(|item| item.argument_expr_idx().to_hir_lazy(self))
                        .collect(),
                )
            }
            SemaRitchieParameterArgumentMatch::Keyed(param, item) => {
                HirLazyCallListItemGroup::Keyed(
                    param.key(),
                    item.as_ref()
                        .map(|item| item.argument_expr_idx().to_hir_lazy(self)),
                )
            }
        }
    }
}
