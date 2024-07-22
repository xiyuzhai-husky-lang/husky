use super::*;
use crate::coercion::HirEagerCoercion;
use husky_hir_ty::ritchie::HirRitchieSimpleParameter;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum HirEagerRitchieArgument {
    Simple(HirRitchieSimpleParameter, HirEagerExprIdx, HirEagerCoercion),
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
                HirRitchieSimpleParameter::from_fly(param, self.db(), self.terms()),
                item.argument_sem_expr_idx().to_hir_eager(self),
                item.coercion_outcome
                    .as_ref()
                    .unwrap()
                    .coercion()
                    .to_hir_eager(self),
            ),
            SemaRitchieArgument::Variadic(_, _) => todo!(),
            SemaRitchieArgument::Keyed(_, _) => todo!(),
        }
    }
}
