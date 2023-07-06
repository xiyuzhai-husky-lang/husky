use self::matcher::*;
use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_call_nonself_arguments_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        ritchie_parameters: &[FluffyTermRitchieParameter],
        ritchie_arguments: impl Iterator<Item = CallListItem> + Clone,
    ) {
        match RitchieParameterArgumentMatcher::new(ritchie_parameters, ritchie_arguments.clone())
            .match_all()
        {
            Ok(matches) => {
                for m in matches {
                    match m {
                        RitchieParameterArgumentMatch::Regular(_, _) => todo!(),
                        RitchieParameterArgumentMatch::Variadic(_, _) => todo!(),
                        RitchieParameterArgumentMatch::Keyed(_, _) => todo!(),
                    }
                }
                // let nonself_parameter_contracted_ty = nonself_parameter_contracted_tys[i];
                // match nonself_parameter_contracted_ty.kind() {
                //     FluffyExplicitParameterKind::Regular => (),
                //     FluffyExplicitParameterKind::Keyed { ident } => todo!(),
                // }
                // self.infer_new_expr_ty_discarded(
                //     nonself_argument.argument_expr_idx(),
                //     ExpectImplicitlyConvertible::new(nonself_parameter_contracted_ty),
                todo!()
            }
            Err(_) => ritchie_arguments.for_each(|ritchie_argument| {
                self.infer_new_expr_ty_discarded(
                    ritchie_argument.argument_expr_idx(),
                    ExpectAnyDerived,
                )
            }),
        }
    }
}

mod matcher {
    use super::*;

    pub enum RitchieParameterArgumentMatch {
        Regular(FluffyTermRitchieRegularParameter, CallListItem),
        Variadic(FluffyTermRitchieVariadicParameter, CallListItem),
        Keyed(FluffyTermRitchieKeyedParameter, CallListItem),
    }

    pub type RitchieParameterArgumentMatchs = SmallVec<[RitchieParameterArgumentMatch; 4]>;

    pub(super) struct RitchieParameterArgumentMatcher<'a, Arguments: Iterator<Item = CallListItem>> {
        ritchie_parameters: &'a [FluffyTermRitchieParameter],
        ritchie_arguments: Arguments,
        ritchie_matches: RitchieParameterArgumentMatchs,
    }

    impl<'a, Arguments: Iterator<Item = CallListItem>> RitchieParameterArgumentMatcher<'a, Arguments> {
        pub(super) fn new(
            ritchie_parameters: &[FluffyTermRitchieParameter],
            ritchie_arguments: Arguments,
        ) -> RitchieParameterArgumentMatcher<'_, impl Iterator<Item = CallListItem>> {
            RitchieParameterArgumentMatcher {
                ritchie_parameters,
                ritchie_arguments,
                ritchie_matches: Default::default(),
            }
        }

        pub(super) fn match_all(&mut self) -> ExprTypeResult<RitchieParameterArgumentMatchs> {
            todo!()
        }
    }
}
