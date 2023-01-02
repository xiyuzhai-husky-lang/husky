use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum UnfinishedExpr {
    Binary {
        lopd: Expr,
        punctuation: BinaryPunctuation,
        punctuation_token_idx: TokenIdx,
    },
    ListItem {
        separator_token_idx: Option<TokenIdx>,
    },
    Prefix {
        punctuation: PrefixPunctuation,
        punctuation_token_idx: TokenIdx,
    },
    List {
        opr: UnfinishedListOpr,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        items: Vec<Expr>,
    },
    LambdaHead {
        inputs: Vec<(RangedIdentifier, Option<ExprIdx>)>,
        start: TextPosition,
    },
    Application {
        function: Expr,
    },
}

impl UnfinishedExpr {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            UnfinishedExpr::Binary {
                punctuation: binary,
                ..
            } => (*binary).into(),
            UnfinishedExpr::Prefix { .. } => Precedence::Prefix,
            UnfinishedExpr::ListItem { .. } | UnfinishedExpr::List { .. } => Precedence::None,
            UnfinishedExpr::LambdaHead { inputs, start } => Precedence::LambdaHead,
            UnfinishedExpr::Application { function } => Precedence::Application,
        }
    }
}
