use parsec::parse_consecutive_list;

use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct TupleStructFieldDeclPattern {
    decorators: Vec<FieldDecorator>,
    visibility: Option<FieldVisibilityExpr>,
    ty: ExprIdx,
}

impl TupleStructFieldDeclPattern {
    pub fn ty(&self) -> ExprIdx {
        self.ty
    }
}

impl<'a, 'b> parsec::ParseFromStream<ExprParseContext<'a, 'b>> for TupleStructFieldDeclPattern {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        let decorators = parse_consecutive_list(ctx)?;
        let visibility = ctx.parse()?;
        let ty = ctx.parse_expr_expected2(None, OriginalExprError::ExpectedFieldType);
        ctx.add_expr_root(ExprRoot::new(ExprRootKind::FieldType, ty));
        Ok(Some(TupleStructFieldDeclPattern {
            decorators,
            visibility,
            ty,
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
