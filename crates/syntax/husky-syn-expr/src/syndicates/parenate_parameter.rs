use super::*;
use either::*;
#[cfg(test)]
use expect_test::*;
use husky_token_data::delimiter::Delimiter;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParenateParameterSyndicate {
    /// ad hoc
    attrs: Vec<()>,
    const_constraint: Option<ConstConstraint>,
    nucleus: ParenateParameterSyndicateNucleus,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstConstraint {
    const_token: ConstRegionalToken,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParenateParameterSyndicateNucleus {
    Simple {
        syn_pattern_root: ParenateParameterSynPatternRoot,
        variables: CurrentSynSymbolIdxRange,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Variadic {
        dot_dot_dot_token: DotDotDotRegionalToken,
        variadic_variant: SynVariadicParameterVariant,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: CurrentVariableIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
    },
    Keyed {
        syn_pattern_root: ParenateParameterSynPatternRoot,
        symbol_modifier_keyword_group: Option<EphemSymbolModifierRegionalTokens>,
        ident_token: IdentRegionalToken,
        variable: CurrentVariableIdx,
        colon: ColonRegionalToken,
        ty: SynExprIdx,
        eq_token: EqRegionalToken,
        // todo: change this to custom enum
        default: Either<UnderscoreRegionalToken, SynExprIdx>,
    },
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SynVariadicParameterVariant {
    Default,
    Vec {
        lbox_token: LboxRegionalToken,
        rbox_token: RboxRegionalToken,
    },
}

impl ParenateParameterSyndicate {
    pub fn nucleus(&self) -> &ParenateParameterSyndicateNucleus {
        &self.nucleus
    }
}

impl<'a> TryParseOptionFromStream<SynDeclExprParser<'a>> for ParenateParameterSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        let const_constraint = ctx.try_parse_option()?;
        let nucleus = if let Some(syn_pattern_root) =
            ctx.try_parse_option::<ParenateParameterSynPatternRoot>()?
        {
            let symbols = ctx
                .pattern_expr_region()
                .pattern_expr_symbols(syn_pattern_root.syn_pattern_idx());
            let access_start = ctx.state().next_regional_token_idx();
            let variables = symbols
                .iter()
                .map(|(ident, pattern_variable_idx)| {
                    CurrentVariableEntry::new(
                        ctx.pattern_expr_region(),
                        access_start,
                        None,
                        CurrentVariableData::SimpleParenateParameter {
                            ident: *ident,
                            pattern_variable_idx: *pattern_variable_idx,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty_expr_idx = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinDelimiteredParameterList(
                    Delimiter::Par,
                )),
                SynExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variables = ctx.define_symbols(
                variables,
                Some(SyndicateTypeConstraint::SimpleParenateParameter {
                    syn_pattern_root,
                    ty: ty_expr_idx,
                }),
            );
            if let Some(eq_token) = ctx.try_parse_option::<EqRegionalToken>()? {
                let SynPatternData::Ident {
                    symbol_modifier_tokens: symbol_modifier_keyword_group,
                    ident_token,
                } = ctx.pattern_expr_region()[syn_pattern_root.syn_pattern_idx()]
                else {
                    todo!()
                };
                // todo: KeyedWithoutDefault
                let default = if let Some(_) = ctx.try_parse_option::<UnderscoreRegionalToken>()? {
                    todo!()
                } else {
                    Right(ctx.parse_expr_expected2(
                        Some(ExprEnvironment::WithinDelimiteredParameterList(
                            Delimiter::Par,
                        )),
                        SynExprRootKind::ParenateParameterDefaultValue {
                            ty_syn_expr_idx: ty_expr_idx,
                        },
                        OriginalSynExprError::ExpectedExplicitParameterDefaultValue,
                    ))
                };
                ParenateParameterSyndicateNucleus::Keyed {
                    syn_pattern_root,
                    symbol_modifier_keyword_group,
                    ident_token,
                    variable: variables.start(),
                    colon,
                    ty: ty_expr_idx,
                    eq_token,
                    default,
                }
            } else {
                ParenateParameterSyndicateNucleus::Simple {
                    syn_pattern_root,
                    variables,
                    colon,
                    ty: ty_expr_idx,
                }
            }
        } else if let Some(dot_dot_dot_token) = ctx.try_parse_option::<DotDotDotRegionalToken>()? {
            let access_start = ctx.state().next_regional_token_idx();
            let variadic_variant = ctx.try_parse()?;
            let symbol_modifier_keyword_group =
                ctx.try_parse_option::<EphemSymbolModifierRegionalTokens>()?;
            let ident_token = ctx
                .try_parse_expected::<IdentRegionalToken, _>(OriginalSynExprError::ExpectedIdent)?;
            let variable = CurrentVariableEntry::new(
                ctx.pattern_expr_region(),
                access_start,
                None,
                CurrentVariableData::VariadicParenateParameter {
                    ident_token,
                    symbol_modifier_keyword_group,
                },
            );
            let colon = ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
            let ty = ctx.parse_expr_expected2(
                Some(ExprEnvironment::WithinDelimiteredParameterList(
                    Delimiter::Par,
                )),
                SynExprRootKind::ExplicitParameterType,
                OriginalSynExprError::ExpectedParameterType,
            );
            let variable = ctx.define_symbol(
                variable,
                Some(SyndicateTypeConstraint::VariadicParenateParameter { ident_token, ty }),
            );
            ParenateParameterSyndicateNucleus::Variadic {
                dot_dot_dot_token,
                variadic_variant,
                symbol_modifier_keyword_group,
                ident_token,
                variable,
                colon,
                ty,
            }
        } else {
            return Ok(None);
        };
        Ok(Some(Self {
            attrs: vec![()], // ad hoc
            const_constraint,
            nucleus,
        }))
    }
}

impl<'a> TryParseOptionFromStream<SynDeclExprParser<'a>> for ConstConstraint {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        sp: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(const_token) = sp.try_parse_option()? else {
            return Ok(None);
        };
        Ok(Some(Self { const_token }))
    }
}

impl<'a, 'b> TryParseFromStream<SynDeclExprParser<'a>> for SynVariadicParameterVariant {
    type Error = SynExprError;

    fn try_parse_from_stream(sp: &mut SynDeclExprParser<'a>) -> Result<Self, Self::Error> {
        if let Some(lbox_token) = sp.try_parse_option::<LboxRegionalToken>()? {
            if let Some(rbox_token) = sp.try_parse_option::<RboxRegionalToken>()? {
                Ok(SynVariadicParameterVariant::Vec {
                    lbox_token,
                    rbox_token,
                })
            } else {
                todo!()
            }
        } else {
            Ok(SynVariadicParameterVariant::Default)
        }
    }
}

#[test]
fn parenate_parameter_syndicate_works() {
    fn t(input: &str, expect: &Expect, db: &::salsa::Db) {
        use salsa::DebugWithDb;

        expect.assert_debug_eq(
            &snippet::try_parse_snippet_in_decl::<ParenateParameterSyndicate>(input, db).debug(db),
        )
    }
    let db = &DB::default();
    t(
        "a: i32",
        &expect![[r#"
        Ok(
            Some(
                ParenateParameterSyndicate {
                    attrs: [
                        (),
                    ],
                    const_constraint: None,
                    nucleus: ParenateParameterSyndicateNucleus::Simple {
                        syn_pattern_root: ParenateParameterSynPatternRoot {
                            syn_pattern_idx: 0,
                        },
                        variables: ArenaIdxRange(
                            0..1,
                        ),
                        colon: ColonRegionalToken(
                            RegionalTokenIdx(
                                2,
                            ),
                        ),
                        ty: 0,
                    },
                },
            ),
        )
    "#]],
        db,
    );
    t(
        "const a: i32",
        &expect![[r#"
        Ok(
            Some(
                ParenateParameterSyndicate {
                    attrs: [
                        (),
                    ],
                    const_constraint: Some(
                        ConstConstraint {
                            const_token: ConstRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                    nucleus: ParenateParameterSyndicateNucleus::Simple {
                        syn_pattern_root: ParenateParameterSynPatternRoot {
                            syn_pattern_idx: 0,
                        },
                        variables: ArenaIdxRange(
                            0..1,
                        ),
                        colon: ColonRegionalToken(
                            RegionalTokenIdx(
                                3,
                            ),
                        ),
                        ty: 0,
                    },
                },
            ),
        )
    "#]],
        db,
    );
    t(
        "const skip: i32 = 5",
        &expect![[r#"
        Ok(
            Some(
                ParenateParameterSyndicate {
                    attrs: [
                        (),
                    ],
                    const_constraint: Some(
                        ConstConstraint {
                            const_token: ConstRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                    nucleus: ParenateParameterSyndicateNucleus::Keyed {
                        syn_pattern_root: ParenateParameterSynPatternRoot {
                            syn_pattern_idx: 0,
                        },
                        symbol_modifier_keyword_group: None,
                        ident_token: IdentRegionalToken {
                            ident: `skip`,
                            regional_token_idx: RegionalTokenIdx(
                                2,
                            ),
                        },
                        variable: 0,
                        colon: ColonRegionalToken(
                            RegionalTokenIdx(
                                3,
                            ),
                        ),
                        ty: 0,
                        eq_token: EqRegionalToken(
                            RegionalTokenIdx(
                                5,
                            ),
                        ),
                        default: Right(
                            1,
                        ),
                    },
                },
            ),
        )
    "#]],
        db,
    );
}
