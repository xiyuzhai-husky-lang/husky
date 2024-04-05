use super::*;
use crate::coersion::HirEagerCoersion;
use husky_hir_ty::ritchie::HirRitchieSimpleParameter;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum HirEagerRitchieArgument {
    Simple(HirRitchieSimpleParameter, HirEagerExprIdx, HirEagerCoersion),
    Variadic,
    Keyed,
}

impl<'a> HirEagerExprBuilder<'a> {
    pub(super) fn new_call_list_arguments(
        &mut self,
        pams: &[SemaRitchieArgument],
    ) -> SmallVec<[HirEagerRitchieArgument; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(&mut self, pam: &SemaRitchieArgument) -> HirEagerRitchieArgument {
        match pam {
            SemaRitchieArgument::Simple(param, item) => HirEagerRitchieArgument::Simple(
                HirRitchieSimpleParameter::from_fly(param, self.db(), self.fly_terms()),
                item.argument_sem_expr_idx().to_hir_eager(self),
                item.coersion_outcome
                    .as_ref()
                    .unwrap()
                    .coersion()
                    .to_hir_eager(self),
            ),
            SemaRitchieArgument::Variadic(_, _) => todo!(),
            SemaRitchieArgument::Keyed(_, _) => todo!(),
        }
    }
}
