use parsec::ParseFromStream;

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
    List {
        opr: UnfinishedListOpr,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        items: Vec<Expr>,
        commas: Vec<TokenIdx>,
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
        todo!()
    }
}

impl UnfinishedExpr {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            UnfinishedExpr::Binary { punctuation, .. } => (*punctuation).into(),
            UnfinishedExpr::Prefix { .. } => Precedence::Prefix,
            UnfinishedExpr::ListItem { .. } | UnfinishedExpr::List { .. } => Precedence::None,
            UnfinishedExpr::LambdaHead { .. } => Precedence::LambdaHead,
            UnfinishedExpr::Application { .. } => Precedence::Application,
        }
    }
}
