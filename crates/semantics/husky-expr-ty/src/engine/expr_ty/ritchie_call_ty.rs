use self::matcher::*;
use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_arguments_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        ritchie_parameters: &[FluffyTermRitchieParameter],
        ritchie_arguments: impl Iterator<Item = CallListItem> + Clone,
    ) {
        match RitchieParameterArgumentMatcher::new(ritchie_parameters, ritchie_arguments.clone())
            .match_all()
        {
            Ok(ritchie_matches) => {
                for ritchie_match in ritchie_matches {
                    match ritchie_match {
                        RitchieParameterArgumentMatch::Regular(param, item) => self
                            .infer_new_expr_ty_discarded(
                                item.argument_expr_idx(),
                                ExpectCoersion::new(param.contract(), param.ty()),
                            ),
                        RitchieParameterArgumentMatch::Variadic(param, items) => {
                            for item in items {
                                self.infer_new_expr_ty_discarded(
                                    item.argument_expr_idx(),
                                    ExpectCoersion::new(param.contract(), param.ty()),
                                )
                            }
                        }
                        RitchieParameterArgumentMatch::Keyed(param, item) => self
                            .infer_new_expr_ty_discarded(
                                item.argument_expr_idx(),
                                ExpectCoersion::new(param.contract(), param.ty()),
                            ),
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
            // use vec to save enum size
            Vec<RegularOrVariadicCallListItem>,
        ),
        Keyed(FluffyTermRitchieKeyedParameter, KeyedCallListItem),
    }

    pub type RitchieParameterArgumentMatchs = SmallVec<[RitchieParameterArgumentMatch; 4]>;

    pub(super) struct RitchieParameterArgumentMatcher<'a, Arguments: Iterator<Item = CallListItem>> {
        ritchie_parameters: &'a [FluffyTermRitchieParameter],
        ritchie_call_items: std::iter::Peekable<Arguments>,
        ritchie_matches: RitchieParameterArgumentMatchs,
    }

    impl<'a, Arguments: Iterator<Item = CallListItem>> RitchieParameterArgumentMatcher<'a, Arguments> {
        pub(super) fn new(
            ritchie_parameters: &[FluffyTermRitchieParameter],
            ritchie_arguments: Arguments,
        ) -> RitchieParameterArgumentMatcher<'_, impl Iterator<Item = CallListItem>> {
            RitchieParameterArgumentMatcher {
                ritchie_parameters,
                ritchie_call_items: ritchie_arguments.peekable(),
                ritchie_matches: Default::default(),
            }
        }

        pub(super) fn match_all(mut self) -> ExprTypeResult<RitchieParameterArgumentMatchs> {
            for ritchie_parameter in self.ritchie_parameters {
                self.match_step(*ritchie_parameter)?
            }
            match self.ritchie_call_items.next() {
                Some(_) => Err(OriginalExprTypeError::UnexpectedArgument)?,
                None => Ok(self.ritchie_matches),
            }
        }

        fn match_step(&mut self, param: FluffyTermRitchieParameter) -> ExprTypeResult<()> {
            match param {
                FluffyTermRitchieParameter::Regular(param) => match self.ritchie_call_items.next() {
                    Some(item) => match item {
                        CallListItem::RegularOrVariadic(item) => Ok(self
                            .ritchie_matches
                            .push(RitchieParameterArgumentMatch::Regular(param, item))),
                        CallListItem::Keyed(_) => todo!(),
                    },
                    None => Err(OriginalExprTypeError::MissingArgument)?,
                },
                FluffyTermRitchieParameter::Variadic(param) => {
                    let mut items = vec![];
                    while let Some(CallListItem::RegularOrVariadic(item)) = self
                        .ritchie_call_items
                        .next_if(|item| matches!(item, CallListItem::RegularOrVariadic(_)))
                    {
                        items.push(item);
                        match item.separator() {
                            CallListSeparator::None | CallListSeparator::Comma(_) => (),
                            CallListSeparator::Semicolon(_) => break,
                        }
                    }
                    Ok(self
                        .ritchie_matches
                        .push(RitchieParameterArgumentMatch::Variadic(param, items)))
                }
                FluffyTermRitchieParameter::Keyed(param) => match param.default() {
                    Some(default) => {
                        if let Some(CallListItem::Keyed(item)) = self.ritchie_call_items
                            .next_if(|arg|
                                matches!(arg, CallListItem::Keyed(item) if item.key() == param.key())
                            ) {
                            Ok(self
                                .ritchie_matches
                                .push(RitchieParameterArgumentMatch::Keyed(param, item)))
                        } else {
                            Ok(())
                        }
                    }
                    None => todo!(),
                },
            }
        }
    }
}
