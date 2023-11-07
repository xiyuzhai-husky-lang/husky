use parsec::{parse_consecutive_list, HasStreamState};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct PropsFieldSyndicate {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ident_token: IdentRegionalToken,
    colon: ColonRegionalToken,
    ty_expr_idx: SynExprIdx,
    initialization: Option<PropsFieldInitialization>,
    variable: CurrentSynSymbolIdx,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PropsFieldInitialization {
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

    pub fn ty_syn_expr_idx(&self) -> SynExprIdx {
        self.ty_expr_idx
    }

    pub fn initialization(&self) -> Option<PropsFieldInitialization> {
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
        let ty_expr_idx = ctx.parse_expr_expected2(
            None,
            SynExprRootKind::PropsStructFieldType { ident_token },
            OriginalSynExprError::ExpectedFieldType,
        );
        let initialization =
            if let Some(colon_eq_token) = ctx.try_parse_option::<ColonEqRegionalToken>()? {
                Some(PropsFieldInitialization::Bind {
                    colon_eq_token,
                    value: ctx.parse_expr_expected2(
                        None,
                        SynExprRootKind::FieldBindInitialValue {
                            ty_syn_expr_idx: ty_expr_idx,
                        },
                        OriginalSynExprError::ExpectedValueForFieldBindInitialization,
                    ),
                })
            } else if let Some(_) = ctx.try_parse_option::<EqRegionalToken>()? {
                todo!()
            } else {
                None
            };
        let access_start = ctx.save_state().next_regional_token_idx();
        let symbol = CurrentSynSymbol::new(
            ctx.pattern_expr_region(),
            access_start,
            None,
            CurrentSynSymbolVariant::FieldVariable { ident_token },
        );
        let variable = ctx.define_symbol(
            symbol,
            Some(SyndicateTypeConstraint::FieldVariable {
                ident_token,
                ty_expr_idx,
            }),
        );
        Ok(Some(PropsFieldSyndicate {
            decorators,
            visibility,
            ident_token,
            colon,
            ty_expr_idx,
            initialization,
            variable,
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct FieldDecorator {}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for FieldDecorator {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pound_token) = ctx.try_parse_option::<PoundRegionalToken>()? else {
            return Ok(None);
        };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub enum FieldVisibilityExpr {
    Pub,
}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for FieldVisibilityExpr {
    type Error = SynExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.try_parse_option::<PubRegionalToken>()? else {
            return Ok(None);
        };
        let Some(lpar_token) = ctx.try_parse_option::<LparRegionalToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub));
        };
        todo!()
    }
}
