use parsec::{parse_consecutive_list, HasStreamState};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb)]
pub struct PropsFieldDeclPattern {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ident_token: IdentToken,
    colon: ColonToken,
    ty_expr_idx: SynExprIdx,
    initialization: Option<PropsFieldInitialization>,
    variable: CurrentSynSymbolIdx,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PropsFieldInitialization {
    Bind {
        colon_eq_token: ColonEqToken,
        value: SynExprIdx,
    },
    Default {},
}

impl PropsFieldDeclPattern {
    pub fn ident(&self) -> Ident {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty_expr_idx(&self) -> SynExprIdx {
        self.ty_expr_idx
    }

    pub fn initialization(&self) -> Option<PropsFieldInitialization> {
        self.initialization
    }
}

impl<'a, 'b> parsec::TryParseOptionFromStream<SynDeclExprParser<'a>> for PropsFieldDeclPattern {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> SynExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.try_parse_option()?;
        let Some(ident_token) = ctx.try_parse_option::<IdentToken>()? else {
            return Ok(None);
        };
        let colon: ColonToken = ctx.try_parse_expected(OriginalExprError::ExpectedColon)?;
        let ty_expr_idx = ctx.parse_expr_expected2(
            None,
            ExprRootKind::PropsStructFieldType { ident_token },
            OriginalExprError::ExpectedFieldType,
        );
        let initialization = if let Some(colon_eq_token) = ctx.try_parse_option::<ColonEqToken>()? {
            Some(PropsFieldInitialization::Bind {
                colon_eq_token,
                value: ctx.parse_expr_expected2(
                    None,
                    ExprRootKind::FieldBindInitialValue { ty_expr_idx },
                    OriginalExprError::ExpectedValueForFieldBindInitialization,
                ),
            })
        } else if let Some(_) = ctx.try_parse_option::<EqToken>()? {
            todo!()
        } else {
            None
        };
        let access_start = ctx.save_state().next_token_idx();
        let symbol = CurrentSynSymbol::new(
            ctx.pattern_expr_region(),
            access_start,
            None,
            CurrentSynSymbolVariant::FieldVariable { ident_token },
        );
        let variable = ctx.define_symbol(
            symbol,
            Some(ObeliskTypeConstraint::FieldVariable {
                ident_token,
                ty_expr_idx,
            }),
        );
        Ok(Some(PropsFieldDeclPattern {
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
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(at_token) = ctx.try_parse_option::<AtToken>()? else {
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
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut SynDeclExprParser<'a>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.try_parse_option::<PubToken>()? else {
            return Ok(None);
        };
        let Some(lpar_token) = ctx.try_parse_option::<LparToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub));
        };
        todo!()
    }
}
