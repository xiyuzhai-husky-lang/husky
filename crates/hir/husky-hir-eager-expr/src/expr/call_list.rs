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
        pams: &[SemRitchieArgument],
    ) -> SmallVec<[HirEagerRitchieArgument; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(&mut self, pam: &SemRitchieArgument) -> HirEagerRitchieArgument {
        match pam {
            SemRitchieArgument::Simple(param, item) => HirEagerRitchieArgument::Simple(
                HirRitchieSimpleParameter::from_fly(param, self.db(), self.terms()),
                item.argument_sem_expr_idx().to_hir_eager(self),
                item.coercion_outcome
                    .as_ref()
                    .unwrap()
                    .coercion()
                    .to_hir_eager(self),
            ),
            SemRitchieArgument::Variadic(_, _) => todo!(),
            SemRitchieArgument::Keyed(_, _) => todo!(),
        }
    }
}
