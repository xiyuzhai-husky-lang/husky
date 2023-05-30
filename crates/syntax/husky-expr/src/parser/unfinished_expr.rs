use parsec::ParseFromStream;
use smallvec::SmallVec;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum UnfinishedExpr {
    Binary {
        lopd: Expr,
        punctuation: BinaryOpr,
        punctuation_token_idx: TokenIdx,
    },
    ListItem {
        separator_token_idx: Option<TokenIdx>,
    },
    Prefix {
        punctuation: PrefixOpr,
        punctuation_token_idx: TokenIdx,
    },
    SimpleList {
        opr: UnfinishedListOpr,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        items: Vec<Expr>,
        commas: Vec<TokenIdx>,
    },
    KeyedArgumentList {
        function: ExprIdx,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        arguments: Vec<Expr>,
        commas: Vec<TokenIdx>,
        keyed_arguments: SmallVec<[KeyedArgumentExpr; 2]>,
    },
    LambdaHead {
        inputs: Vec<(RangedIdent, Option<ExprIdx>)>,
        start: TextPosition,
    },
    Application {
        function: Expr,
    },
}

impl<'a, 'b> ParseFromStream<ExprParseContext<'a, 'b>> for HtmlArgumentExpr {
    type Error = ExprError;

    fn parse_from_without_guaranteed_rollback(
        sp: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some(lcurl) = sp.parse::<LeftCurlyBraceToken>()? {
            Ok(Some(HtmlArgumentExpr::Shortened {
                lcurl,
                property_ident: sp.parse_expected(OriginalExprError::HtmlTodo)?,
                rcurl: sp.parse_expected(OriginalExprError::HtmlTodo)?,
            }))
        } else if let Some(argument_ident) = sp.parse::<IdentToken>()? {
            Ok(Some(HtmlArgumentExpr::Expanded {
                property_ident: argument_ident,
                eq: sp.parse_expected(OriginalExprError::HtmlTodo)?,
                lcurl: sp.parse_expected(OriginalExprError::HtmlTodo)?,
                expr: sp.parse_expr_expected2(
                    None,
                    ExprRootKind::HtmlArgumentExpr,
                    OriginalExprError::HtmlTodo,
                ),
                rcurl: sp.parse_expected(OriginalExprError::HtmlTodo)?,
            }))
        } else {
            Ok(None)
        }
    }
}

impl UnfinishedExpr {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            UnfinishedExpr::Binary { punctuation, .. } => (*punctuation).into(),
            UnfinishedExpr::Prefix { .. } => Precedence::Prefix,
            UnfinishedExpr::ListItem { .. }
            | UnfinishedExpr::SimpleList { .. }
            | UnfinishedExpr::KeyedArgumentList { .. } => Precedence::None,
            UnfinishedExpr::LambdaHead { .. } => Precedence::LambdaHead,
            UnfinishedExpr::Application { .. } => Precedence::Application,
        }
    }
}
