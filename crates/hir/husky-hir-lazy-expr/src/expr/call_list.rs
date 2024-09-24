use super::*;
use husky_sem_expr::SemRitchieArgument;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[enum_class::from_variants]
pub enum HirLazyCallListArgument {
    Simple(HirLazyExprIdx),
    Variadic(Vec<HirLazyExprIdx>),
    Keyed(Ident, Option<HirLazyExprIdx>),
}

impl<'a> HirLazyExprBuilder<'a> {
    pub(super) fn new_call_list_item_groups(
        &mut self,
        pams: &[SemRitchieArgument],
    ) -> SmallVec<[HirLazyCallListArgument; 4]> {
        pams.iter()
            .map(|pam| self.new_call_list_item_group(pam))
            .collect()
    }

    fn new_call_list_item_group(&mut self, pam: &SemRitchieArgument) -> HirLazyCallListArgument {
        match pam {
            SemRitchieArgument::Simple(_, item) => {
                HirLazyCallListArgument::Simple(item.argument_sem_expr_idx().to_hir_lazy(self))
            }
            SemRitchieArgument::Variadic(_, items) => HirLazyCallListArgument::Variadic(
                items
                    .iter()
                    .map(|item| item.argument_expr_idx().to_hir_lazy(self))
                    .collect(),
            ),
            SemRitchieArgument::Keyed(param, item) => HirLazyCallListArgument::Keyed(
                param.key(),
                item.as_ref()
                    .map(|item| item.argument_expr_idx().to_hir_lazy(self)),
            ),
        }
    }
}
