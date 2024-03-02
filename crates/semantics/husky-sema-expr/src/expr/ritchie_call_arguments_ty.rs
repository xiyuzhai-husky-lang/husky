pub use self::matcher::*;

use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_ritchie_arguments_ty(
        &mut self,
        expr_idx: SynExprIdx,
        ritchie_parameters: &[FlyRitchieParameter],
        ritchie_arguments: impl Iterator<Item = SynCallListItem> + Clone,
    ) -> SemaExprDataResult<RitchieParameterArgumentMatches> {
        RitchieParameterArgumentMatcher::new(ritchie_parameters, ritchie_arguments.clone(), self)
            .match_all()
            .map_err(|match_error| {
                OriginalSemaExprDataError::RitchieParameterArgumentMismatch {
                    match_error,
                    ritchie_arguments: ritchie_arguments
                        .map(|ritchie_argument| {
                            self.build_sema_expr(
                                ritchie_argument.argument_expr_idx(),
                                ExpectAnyDerived,
                            )
                        })
                        .collect(),
                }
                .into()
            })
    }
}

mod matcher {
    use super::*;
    use thiserror::Error;

    #[salsa::debug_with_db]
    #[derive(Debug, Error, PartialEq, Eq)]
    pub enum RitchieParameterArgumentMatchError {
        #[error("unexpected argument")]
        UnexpectedArgument,
        #[error("missing argument")]
        MissingArgument,
    }
    pub type RitchieParameterArgumentMatchResult<T> = Result<T, RitchieParameterArgumentMatchError>;

    #[salsa::debug_with_db]
    #[derive(Debug, PartialEq, Eq)]
    pub enum SemaRitchieParameterArgumentMatch {
        Simple(FlyRitchieSimpleParameter, SemaRegularCallListItem),
        Variadic(
            FlyRitchieVariadicParameter,
            // use vec to save enum size
            Vec<SemaVariadicCallListItem>,
        ),
        Keyed(FlyRitchieKeyedParameter, Option<SemaKeyedCallListItem>),
    }

    pub type RitchieParameterArgumentMatches = SmallVec<[SemaRitchieParameterArgumentMatch; 4]>;

    pub(super) struct RitchieParameterArgumentMatcher<
        'a,
        'b,
        Arguments: Iterator<Item = SynCallListItem>,
    > {
        ritchie_parameters: &'b [FlyRitchieParameter],
        ritchie_call_items: std::iter::Peekable<Arguments>,
        ritchie_matches: RitchieParameterArgumentMatches,
        engine: &'b mut SemaExprEngine<'a>,
    }

    impl<'a, 'b, Arguments: Iterator<Item = SynCallListItem>>
        RitchieParameterArgumentMatcher<'a, 'b, Arguments>
    {
        pub(super) fn new(
            ritchie_parameters: &'b [FlyRitchieParameter],
            ritchie_arguments: Arguments,
            engine: &'b mut SemaExprEngine<'a>,
        ) -> RitchieParameterArgumentMatcher<'a, 'b, impl Iterator<Item = SynCallListItem>>
        {
            RitchieParameterArgumentMatcher {
                ritchie_parameters,
                ritchie_call_items: ritchie_arguments.peekable(),
                ritchie_matches: Default::default(),
                engine,
            }
        }

        pub(super) fn match_all(
            mut self,
        ) -> RitchieParameterArgumentMatchResult<RitchieParameterArgumentMatches> {
            for ritchie_parameter in self.ritchie_parameters {
                self.match_step(*ritchie_parameter)?
            }
            match self.ritchie_call_items.next() {
                Some(_) => Err(RitchieParameterArgumentMatchError::UnexpectedArgument)?,
                None => Ok(self.ritchie_matches),
            }
        }

        fn match_step(
            &mut self,
            param: FlyRitchieParameter,
        ) -> RitchieParameterArgumentMatchResult<()> {
            match param {
                FlyRitchieParameter::Regular(param) => match self.ritchie_call_items.next() {
                    Some(item) => match item {
                        SynCallListItem::RegularOrVariadic(item) => {
                            let (argument_sema_expr_idx, coersion) =
                                self.engine.build_sema_expr_with_outcome(
                                    item.argument_expr_idx(),
                                    ExpectCoersion::new(param.contract(), param.ty()),
                                );
                            let item = SemaRegularCallListItem::new(
                                argument_sema_expr_idx,
                                coersion,
                                item.separator(),
                            );
                            Ok(self
                                .ritchie_matches
                                .push(SemaRitchieParameterArgumentMatch::Simple(param, item)))
                        }
                        SynCallListItem::Keyed(_) => todo!(),
                    },
                    None => Err(RitchieParameterArgumentMatchError::MissingArgument)?,
                },
                FlyRitchieParameter::Variadic(param) => {
                    let mut items = vec![];
                    while let Some(SynCallListItem::RegularOrVariadic(item)) = self
                        .ritchie_call_items
                        .next_if(|item| matches!(item, SynCallListItem::RegularOrVariadic(_)))
                    {
                        let (argument_sema_expr_idx, coersion_outcome) =
                            self.engine.build_sema_expr_with_outcome(
                                item.argument_expr_idx(),
                                ExpectCoersion::new(param.contract(), param.ty()),
                            );
                        items.push(SemaVariadicCallListItem::new(
                            argument_sema_expr_idx,
                            coersion_outcome,
                            item.separator(),
                        ));
                        match item.separator() {
                            CallListSeparator::None | CallListSeparator::Comma(_) => (),
                            CallListSeparator::Semicolon(_) => break,
                        }
                    }
                    Ok(self
                        .ritchie_matches
                        .push(SemaRitchieParameterArgumentMatch::Variadic(param, items)))
                }
                FlyRitchieParameter::Keyed(param) => match param.has_default() {
                    true => {
                        let item =  if let Some(SynCallListItem::Keyed(item)) = self.ritchie_call_items
                            .next_if(|arg|
                                matches!(arg, SynCallListItem::Keyed(item) if item.key() == param.key())
                            ) {
                            Some(self.engine.build_sema_keyed_call_list_item(item, param))
                        } else {
                            None
                        };
                        Ok(self
                            .ritchie_matches
                            .push(SemaRitchieParameterArgumentMatch::Keyed(param, item)))
                    }
                    false => todo!(),
                },
            }
        }
    }
}
