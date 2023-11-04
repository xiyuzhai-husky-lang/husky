pub use self::matcher::*;

use super::*;

impl<'a> SemaExprEngine<'a> {
    pub(super) fn calc_ritchie_arguments_ty(
        &mut self,
        expr_idx: SynExprIdx,
        ritchie_parameters: &[FluffyTermRitchieParameter],
        ritchie_arguments: impl Iterator<Item = SynCallListItem> + Clone,
    ) -> SemaExprDataResult<RitchieParameterArgumentMatches> {
        match RitchieParameterArgumentMatcher::new(
            ritchie_parameters,
            ritchie_arguments.clone(),
            self,
        )
        .match_all()
        {
            Ok(ritchie_matches) => {
                // for ritchie_match in &ritchie_matches {
                //     match ritchie_match {
                //         RitchieParameterArgumentMatch::Regular(param, item) => self
                //             .build_new_expr_ty_discarded(
                //                 item.argument_expr_idx(),
                //                 ExpectCoersion::new(param.contract(), param.ty()),
                //             ),
                //         RitchieParameterArgumentMatch::Variadic(param, items) => {
                //             for item in items {
                //                 self.build_new_expr_ty_discarded(
                //                     item.argument_expr_idx(),
                //                     ExpectCoersion::new(param.contract(), param.ty()),
                //                 )
                //             }
                //         }
                //         RitchieParameterArgumentMatch::Keyed(param, item) => self
                //             .build_new_expr_ty_discarded(
                //                 item.argument_expr_idx(),
                //                 ExpectCoersion::new(param.contract(), param.ty()),
                //             ),
                //     }
                // }
                Ok(ritchie_matches)
            }
            Err(match_error) => {
                let ritchie_arguments = ritchie_arguments
                    .map(|ritchie_argument| {
                        self.build_sema_expr(ritchie_argument.argument_expr_idx(), ExpectAnyDerived)
                    })
                    .collect();
                Err(
                    OriginalSemaExprDataError::RitchieParameterArgumentMismatch {
                        match_error,
                        ritchie_arguments,
                    }
                    .into(),
                )
            }
        }
    }
}

mod matcher {
    use super::*;
    use thiserror::Error;

    #[derive(Debug, Error, PartialEq, Eq)]
    #[salsa::debug_with_db(db = SemaExprDb)]
    pub enum RitchieParameterArgumentMatchError {
        #[error("unexpected argument")]
        UnexpectedArgument,
        #[error("missing argument")]
        MissingArgument,
    }
    pub type RitchieParameterArgumentMatchResult<T> = Result<T, RitchieParameterArgumentMatchError>;

    #[derive(Debug, PartialEq, Eq)]
    pub enum SemaRitchieParameterArgumentMatch {
        Regular(FluffyTermRitchieRegularParameter, SemaRegularCallListItem),
        Variadic(
            FluffyTermRitchieVariadicParameter,
            // use vec to save enum size
            Vec<SemaVariadicCallListItem>,
        ),
        Keyed(FluffyTermRitchieKeyedParameter, SemaKeyedCallListItem),
    }

    pub type RitchieParameterArgumentMatches = SmallVec<[SemaRitchieParameterArgumentMatch; 4]>;

    pub(super) struct RitchieParameterArgumentMatcher<
        'a,
        'b,
        Arguments: Iterator<Item = SynCallListItem>,
    > {
        ritchie_parameters: &'b [FluffyTermRitchieParameter],
        ritchie_call_items: std::iter::Peekable<Arguments>,
        ritchie_matches: RitchieParameterArgumentMatches,
        engine: &'b mut SemaExprEngine<'a>,
    }

    impl<'a, 'b, Arguments: Iterator<Item = SynCallListItem>>
        RitchieParameterArgumentMatcher<'a, 'b, Arguments>
    {
        pub(super) fn new(
            ritchie_parameters: &'b [FluffyTermRitchieParameter],
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
            param: FluffyTermRitchieParameter,
        ) -> RitchieParameterArgumentMatchResult<()> {
            match param {
                FluffyTermRitchieParameter::Regular(param) => match self.ritchie_call_items.next() {
                    Some(item) => match item {
                        SynCallListItem::RegularOrVariadic(item) =>{
                            let argument_sema_expr_idx = self.engine
                                        .build_sema_expr (
                                            item.argument_expr_idx(),
                                            ExpectCoersion::new(param.contract(), param.ty()),
                                        );
                            let item = SemaRegularCallListItem::new(argument_sema_expr_idx, item.separator());
                            Ok(self
                            .ritchie_matches
                            .push(SemaRitchieParameterArgumentMatch::Regular(param, item)))},
                        SynCallListItem::Keyed(_) => todo!(),
                    },
                    None => Err(RitchieParameterArgumentMatchError::MissingArgument)?,
                },
                FluffyTermRitchieParameter::Variadic(param) => {
                    let mut items = vec![];
                    while let Some(SynCallListItem::RegularOrVariadic(item)) = self
                        .ritchie_call_items
                        .next_if(|item| matches!(item, SynCallListItem::RegularOrVariadic(_)))
                    {
                        let argument_sema_expr_idx = self.engine
                                    .build_sema_expr (
                                        item.argument_expr_idx(),
                                        ExpectCoersion::new(param.contract(), param.ty()),
                                    );
                        items.push(SemaVariadicCallListItem::new(argument_sema_expr_idx, item.separator()));
                        match item.separator() {
                            CallListSeparator::None | CallListSeparator::Comma(_) => (),
                            CallListSeparator::Semicolon(_) => break,
                        }
                    }
                    Ok(self
                        .ritchie_matches
                        .push(SemaRitchieParameterArgumentMatch::Variadic(param, items)))
                }
                FluffyTermRitchieParameter::Keyed(param) => match param.has_default() {
                    true => {
                        if let Some(SynCallListItem::Keyed(item)) = self.ritchie_call_items
                            .next_if(|arg|
                                matches!(arg, SynCallListItem::Keyed(item) if item.key() == param.key())
                            ) {
                            let item = self.engine.build_sema_keyed_call_list_item(item, param);
                            Ok(self
                                .ritchie_matches
                                .push(SemaRitchieParameterArgumentMatch::Keyed(param, item)))
                        } else {
                            Ok(())
                        }
                    }
                    false => todo!(),
                },
            }
        }
    }
}
