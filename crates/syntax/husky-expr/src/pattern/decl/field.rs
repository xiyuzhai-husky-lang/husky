use parsec::parse_consecutive_list;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct FieldDeclPattern {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ident_token: IdentToken,
    colon: ColonToken,
    ty: ExprIdx,
    initialization: Option<RegularStructFieldInitialization>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegularStructFieldInitialization {
    Bind {
        colon_eq_token: ColonEqToken,
        value: ExprIdx,
    },
    Default {},
}

impl<'a, 'b> ParseFromStream<ExprParseContext<'a, 'b>> for RegularStructFieldInitialization {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        parser: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some(colon_eq_token) = parser.parse::<ColonEqToken>()? {
            Ok(Some(RegularStructFieldInitialization::Bind {
                colon_eq_token,
                value: parser.parse_expr_expected2(
                    None,
                    OriginalExprError::ExpectedValueForFieldBindInitialization,
                ),
            }))
        } else if let Some(_) = parser.parse::<EqToken>()? {
            todo!()
        } else {
            Ok(None)
        }
    }
}

impl FieldDeclPattern {
    pub fn ident(&self) -> Ident {
        self.ident_token.ident()
    }

    pub fn colon(&self) -> ColonToken {
        self.colon
    }

    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}

impl<'a, 'b> parsec::ParseFromStream<ExprParseContext<'a, 'b>> for FieldDeclPattern {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.parse()?;
        let Some(ident_token) = ctx.parse::<IdentToken>()? else {
                return Ok(None)
            };
        let colon: ColonToken = ctx.parse_expected(OriginalExprError::ExpectedColon)?;
        let ty = ctx.parse_expr_expected2(None, OriginalExprError::ExpectedFieldType);
        ctx.add_expr_root(ExprRoot::new(ExprRootKind::FieldType, ty));
        let initialization = ctx.parse()?;
        Ok(Some(FieldDeclPattern {
            decorators,
            visibility,
            ident_token,
            colon,
            ty,
            initialization,
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct FieldDecorator {}

impl<'a, 'b> parsec::ParseFromStream<ExprParseContext<'a, 'b>> for FieldDecorator {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(at_token) = ctx.parse::<AtToken>()? else {
            return Ok(None)
        };
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum FieldVisibilityExpr {
    Pub,
}

impl<'a, 'b> parsec::ParseFromStream<ExprParseContext<'a, 'b>> for FieldVisibilityExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.parse::<PubToken>()? else {
            return Ok(None)
        };
        let Some(lpar_token) = ctx.parse::<LeftParenthesisToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub))
        };
        todo!()
    }
}
