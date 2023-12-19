use super::*;
use crate::coersion::HirEagerCoersion;
use husky_hir_ty::ritchie::HirRitchieRegularParameter;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum HirEagerRitchieParameterArgumentMatch {
    Regular(
        HirRitchieRegularParameter,
        HirEagerExprIdx,
        HirEagerCoersion,
    ),
    Variadic,
    Keyed,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_call_list_item_groups(
        &mut self,
        pams: &[SemaRitchieParameterArgumentMatch],
    ) -> SmallVec<[HirEagerRitchieParameterArgumentMatch; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(
        &mut self,
        pam: &SemaRitchieParameterArgumentMatch,
    ) -> HirEagerRitchieParameterArgumentMatch {
        match pam {
            SemaRitchieParameterArgumentMatch::Regular(param, item) => {
                HirEagerRitchieParameterArgumentMatch::Regular(
                    HirRitchieRegularParameter::from_fluffy(param, self.db(), self.fluffy_terms()),
                    item.argument_sema_expr_idx().to_hir_eager(self),
                    item.coersion_outcome
                        .as_ref()
                        .unwrap()
                        .coersion()
                        .to_hir_eager(self),
                )
            }
            SemaRitchieParameterArgumentMatch::Variadic(_, _) => todo!(),
            SemaRitchieParameterArgumentMatch::Keyed(_, _) => todo!(),
        }
    }
}
