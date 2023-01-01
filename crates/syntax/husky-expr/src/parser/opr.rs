use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum UnfinishedExpr {
    Binary {
        lopd: Expr,
        binary: BinaryPunctuation,
        binary_token_idx: TokenIdx,
    },
    ListItem {
        separator_token_idx: Option<TokenIdx>,
    },
    Prefix {
        prefix: PrefixPunctuation,
        prefix_token_idx: TokenIdx,
    },
    List {
        opr: ListOpr,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        items: Vec<Expr>,
    },
    LambdaHead {
        inputs: Vec<(RangedIdentifier, Option<ExprIdx>)>,
        start: TextPosition,
    },
    Dot {
        dot_token_idx: TokenIdx,
    },
}

impl UnfinishedExpr {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            UnfinishedExpr::Binary { binary, .. } => (*binary).into(),
            UnfinishedExpr::Prefix { .. } => Precedence::Prefix,
            UnfinishedExpr::ListItem { .. } | UnfinishedExpr::List { .. } => Precedence::None,
            UnfinishedExpr::LambdaHead { inputs, start } => Precedence::LambdaHead,
            UnfinishedExpr::Dot { dot_token_idx } => Precedence::Dot,
        }
    }
}
