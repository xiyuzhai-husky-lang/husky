use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum StackOpr {
    Binary {
        binary: BinaryOpr,
        binary_token_idx: TokenIdx,
    },
    ListItem {
        separator_token_idx: Option<TokenIdx>,
    },
    Prefix {
        prefix: PrefixOpr,
        prefix_token_idx: TokenIdx,
    },
    ListStart {
        bra: Bracket,
        bra_token_idx: TokenIdx,
        attr: ListStartAttr,
    },
    LambdaHead {
        inputs: Vec<(RangedIdentifier, Option<ExprIdx>)>,
        start: TextPosition,
    },
    Dot {
        dot_token_idx: TokenIdx,
    },
}

impl StackOpr {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            StackOpr::Binary { binary, .. } => (*binary).into(),
            StackOpr::Prefix { .. } => Precedence::Prefix,
            StackOpr::ListItem { .. } | StackOpr::ListStart { .. } => Precedence::None,
            StackOpr::LambdaHead { inputs, start } => Precedence::LambdaHead,
            StackOpr::Dot { dot_token_idx } => todo!(),
        }
    }
}
