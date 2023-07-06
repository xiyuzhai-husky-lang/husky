use self::matcher::*;
use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_arguments_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        ritchie_parameters: &[FluffyTermRitchieParameter],
        ritchie_arguments: impl Iterator<Item = CallListItem> + Clone,
    ) {
        match RitchieParameterArgumentMatcher::new(ritchie_parameters, ritchie_arguments.clone())
            .match_all()
        {
            Ok(ritchie_matches) => {
                for ritchie_match in ritchie_matches {
                    match ritchie_match {
                        RitchieParameterArgumentMatch::Regular(param, arg) => self
                            .infer_new_expr_ty_discarded(
                                arg.argument_expr_idx(),
                                ExpectImplicitlyConvertible::new(param.contract(), param.ty()),
                            ),
                        RitchieParameterArgumentMatch::Variadic(_, _) => todo!(),
                        RitchieParameterArgumentMatch::Keyed(_, _) => todo!(),
                    }
                }
            }
            Err(_) => {
                println!("Err(_)");
                ritchie_arguments.for_each(|ritchie_argument| {
                    self.infer_new_expr_ty_discarded(
                        ritchie_argument.argument_expr_idx(),
                        ExpectAnyDerived,
                    )
                })
            }
        }
    }
}

mod matcher {
    use super::*;

    pub enum RitchieParameterArgumentMatch {
        Regular(
            FluffyTermRitchieRegularParameter,
            RegularOrVariadicCallListItem,
        ),
        Variadic(
            FluffyTermRitchieVariadicParameter,
            RegularOrVariadicCallListItem,
        ),
        Keyed(FluffyTermRitchieKeyedParameter, KeyedCallListItem),
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

        pub(super) fn match_all(mut self) -> ExprTypeResult<RitchieParameterArgumentMatchs> {
            for ritchie_parameter in self.ritchie_parameters {
                self.match_step(*ritchie_parameter)?
            }
            match self.ritchie_arguments.next() {
                Some(_) => todo!("unexpected"),
                None => Ok(self.ritchie_matches),
            }
        }

        fn match_step(
            &mut self,
            ritchie_parameter: FluffyTermRitchieParameter,
        ) -> ExprTypeResult<()> {
            match ritchie_parameter {
                FluffyTermRitchieParameter::Regular(ritchie_parameter) => {
                    match self.ritchie_arguments.next() {
                        Some(ritchie_argument) => match ritchie_argument {
                            CallListItem::RegularOrVariadic(ritchie_argument) => Ok(self
                                .ritchie_matches
                                .push(RitchieParameterArgumentMatch::Regular(
                                    ritchie_parameter,
                                    ritchie_argument,
                                ))),
                            CallListItem::Keyed(_) => todo!(),
                        },
                        None => todo!(),
                    }
                }
                FluffyTermRitchieParameter::Variadic(_) => todo!(),
                FluffyTermRitchieParameter::Keyed(_) => todo!(),
            }
        }
    }
}
