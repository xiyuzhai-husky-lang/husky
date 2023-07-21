use parsec::parse_consecutive_list;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct TupleFieldDeclPattern {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ty: ExprIdx,
}

impl TupleFieldDeclPattern {
    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}

impl<'a, 'b> parsec::TryParseOptionFromStream<ExprParseContext<'a, 'b>> for TupleFieldDeclPattern {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.try_parse_option()?;
        let ty = ctx.parse_expr_expected2(
            None,
            ExprRootKind::TupleStructFieldType,
            OriginalExprError::ExpectedFieldType,
        );
        Ok(Some(TupleFieldDeclPattern {
            decorators,
            visibility,
            ty,
        }))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct FieldDecorator {}

impl<'a, 'b> parsec::TryParseOptionFromStream<ExprParseContext<'a, 'b>> for FieldDecorator {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(at_token) = ctx.try_parse_option::<AtToken>()? else {
            return Ok(None)
        };
        todo!()
    }
}

// todo: repetitive
// merge with struct field?
#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum FieldVisibilityExpr {
    Pub,
}

impl<'a, 'b> parsec::TryParseOptionFromStream<ExprParseContext<'a, 'b>> for FieldVisibilityExpr {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        let Some(pub_token) = ctx.try_parse_option::<PubToken>()? else {
            return Ok(None)
        };
        let Some(lpar_token) = ctx.try_parse_option::<LeftParenthesisToken>()? else {
            return Ok(Some(FieldVisibilityExpr::Pub))
        };
        todo!()
    }
}
