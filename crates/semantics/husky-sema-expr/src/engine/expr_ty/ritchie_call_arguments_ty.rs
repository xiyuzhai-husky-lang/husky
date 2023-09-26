pub use self::matcher::*;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_arguments_ty(
        &mut self,
        expr_idx: SynExprIdx,
        ritchie_parameters: &[FluffyTermRitchieParameter],
        ritchie_arguments: impl Iterator<Item = SynCallListItem> + Clone,
    ) -> SemaExprResult<RitchieParameterArgumentMatches> {
        match RitchieParameterArgumentMatcher::new(ritchie_parameters, ritchie_arguments.clone())
            .match_all()
        {
            Ok(ritchie_matches) => {
                for ritchie_match in &ritchie_matches {
                    match ritchie_match {
                        RitchieParameterArgumentMatch::Regular(param, item) => self
                            .build_new_expr_ty_discarded(
                                item.argument_expr_idx(),
                                ExpectCoersion::new(param.contract(), param.ty()),
                            ),
                        RitchieParameterArgumentMatch::Variadic(param, items) => {
                            for item in items {
                                self.build_new_expr_ty_discarded(
                                    item.argument_expr_idx(),
                                    ExpectCoersion::new(param.contract(), param.ty()),
                                )
                            }
                        }
                        RitchieParameterArgumentMatch::Keyed(param, item) => self
                            .build_new_expr_ty_discarded(
                                item.argument_expr_idx(),
                                ExpectCoersion::new(param.contract(), param.ty()),
                            ),
                    }
                }
                Ok(ritchie_matches)
            }
            Err(e) => {
                ritchie_arguments.for_each(|ritchie_argument| {
                    self.build_new_expr_ty_discarded(
                        ritchie_argument.argument_expr_idx(),
                        ExpectAnyDerived,
                    )
                });
                Err(e)
            }
        }
    }
}

mod matcher {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    pub enum RitchieParameterArgumentMatch {
        Regular(
            FluffyTermRitchieRegularParameter,
            SynRegularOrVariadicCallListItem,
        ),
        Variadic(
            FluffyTermRitchieVariadicParameter,
            // use vec to save enum size
            Vec<SynRegularOrVariadicCallListItem>,
        ),
        Keyed(FluffyTermRitchieKeyedParameter, KeyedCallListItem),
    }

    pub type RitchieParameterArgumentMatches = SmallVec<[RitchieParameterArgumentMatch; 4]>;

    pub(super) struct RitchieParameterArgumentMatcher<
        'a,
        Arguments: Iterator<Item = SynCallListItem>,
    > {
        ritchie_parameters: &'a [FluffyTermRitchieParameter],
        ritchie_call_items: std::iter::Peekable<Arguments>,
        ritchie_matches: RitchieParameterArgumentMatches,
    }

    impl<'a, Arguments: Iterator<Item = SynCallListItem>>
        RitchieParameterArgumentMatcher<'a, Arguments>
    {
        pub(super) fn new(
            ritchie_parameters: &[FluffyTermRitchieParameter],
            ritchie_arguments: Arguments,
        ) -> RitchieParameterArgumentMatcher<'_, impl Iterator<Item = SynCallListItem>> {
            RitchieParameterArgumentMatcher {
                ritchie_parameters,
                ritchie_call_items: ritchie_arguments.peekable(),
                ritchie_matches: Default::default(),
            }
        }

        pub(super) fn match_all(mut self) -> SemaExprResult<RitchieParameterArgumentMatches> {
            for ritchie_parameter in self.ritchie_parameters {
                self.match_step(*ritchie_parameter)?
            }
            match self.ritchie_call_items.next() {
                Some(_) => Err(OriginalSemaExprError::UnexpectedArgument)?,
                None => Ok(self.ritchie_matches),
            }
        }

        fn match_step(&mut self, param: FluffyTermRitchieParameter) -> SemaExprResult<()> {
            match param {
                FluffyTermRitchieParameter::Regular(param) => match self.ritchie_call_items.next() {
                    Some(item) => match item {
                        SynCallListItem::RegularOrVariadic(item) => Ok(self
                            .ritchie_matches
                            .push(RitchieParameterArgumentMatch::Regular(param, item))),
                        SynCallListItem::Keyed(_) => todo!(),
                    },
                    None => Err(OriginalSemaExprError::MissingArgument)?,
                },
                FluffyTermRitchieParameter::Variadic(param) => {
                    let mut items = vec![];
                    while let Some(SynCallListItem::RegularOrVariadic(item)) = self
                        .ritchie_call_items
                        .next_if(|item| matches!(item, SynCallListItem::RegularOrVariadic(_)))
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
                        if let Some(SynCallListItem::Keyed(item)) = self.ritchie_call_items
                            .next_if(|arg|
                                matches!(arg, SynCallListItem::Keyed(item) if item.key() == param.key())
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
