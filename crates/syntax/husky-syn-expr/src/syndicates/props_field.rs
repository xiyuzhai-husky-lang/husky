use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PropsFieldSyndicate {
    decorators: Vec<PropsFieldAttr>,
    visibility: Option<FieldVisibilityExpr>,
    ident_token: IdentRegionalToken,
    colon: ColonRegionalToken,
    ty: SynExprIdx,
    initialization: Option<PropsFieldSynInitialization>,
    variable: CurrentVariableIdx,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PropsFieldSynInitialization {
    Bind {
        colon_eq_token: ColonEqRegionalToken,
        value: SynExprIdx,
    },
    Default {},
}

impl PropsFieldSyndicate {
    pub fn ident(&self) -> Ident {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonRegionalToken {
        self.colon
    }

    pub fn ty(&self) -> SynExprIdx {
        self.ty
    }

    pub fn initialization(&self) -> Option<PropsFieldSynInitialization> {
        self.initialization
    }
}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for PropsFieldSyndicate {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.try_parse_option()?;
        let Some(ident_token) = ctx.try_parse_option::<IdentRegionalToken>()? else {
            return Ok(None);
        };
        let colon: ColonRegionalToken =
            ctx.try_parse_expected(OriginalSynExprError::ExpectedColon)?;
        let ty_syn_expr_idx = ctx.parse_expr_expected2(
            None,
            SynExprRootKind::PropsStructFieldType { ident_token },
            OriginalSynExprError::ExpectedFieldType,
        );
        let initialization =
            if let Some(colon_eq_token) = ctx.try_parse_option::<ColonEqRegionalToken>()? {
                Some(PropsFieldSynInitialization::Bind {
                    colon_eq_token,
                    value: ctx.parse_expr_expected2(
                        None,
                        SynExprRootKind::FieldBindInitialValue { ty_syn_expr_idx },
                        OriginalSynExprError::ExpectedValueForFieldBindInitialization,
                    ),
                })
            } else if let Some(_) = ctx.try_parse_option::<EqRegionalToken>()? {
                todo!()
            } else {
                None
            };
        let access_start = ctx.state().next_regional_token_idx();
        let symbol = CurrentVariableEntry::new(
            ctx.pattern_expr_region(),
            access_start,
            None,
            CurrentVariableData::FieldVariable { ident_token },
        );
        let variable = ctx.define_symbol(
            symbol,
            Some(SyndicateTypeConstraint::FieldVariable {
                ident_token,
                ty_expr_idx: ty_syn_expr_idx,
            }),
        );
        Ok(Some(PropsFieldSyndicate {
            decorators,
            visibility,
            ident_token,
            colon,
            ty: ty_syn_expr_idx,
            initialization,
            variable,
        }))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PropsFieldAttr {
    pound_token: PoundRegionalToken,
    ident: Ident,
}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for PropsFieldAttr {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pound_token) = ctx.try_parse_option::<PoundRegionalToken>()? else {
            return Ok(None);
        };
        match ctx.next() {
            Some(&token_data) => match token_data {
                TokenData::Ident(ident) => Ok(Some(PropsFieldAttr { pound_token, ident })),
                TokenData::Keyword(_)
                | TokenData::Label(_)
                | TokenData::Punctuation(_)
                | TokenData::WordOpr(_)
                | TokenData::Literal(_) => Ok(None),
                TokenData::Error(e) => return Err(e.into()),
            },
            None => Ok(None),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldVisibilityExpr {
    Pub { pub_token: PubRegionalToken },
}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for FieldVisibilityExpr {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.try_parse_option::<PubRegionalToken>()? else {
            return Ok(None);
        };
        let Some(_lpar_token) = ctx.try_parse_option::<LparRegionalToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub { pub_token }));
        };
        todo!()
    }
}
