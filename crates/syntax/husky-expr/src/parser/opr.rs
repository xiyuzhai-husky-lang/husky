use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum PartialOpn {
    Binary {
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

impl PartialOpn {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            PartialOpn::Binary { binary, .. } => (*binary).into(),
            PartialOpn::Prefix { .. } => Precedence::Prefix,
            PartialOpn::ListItem { .. } | PartialOpn::ListStart { .. } => Precedence::None,
            PartialOpn::LambdaHead { inputs, start } => Precedence::LambdaHead,
            PartialOpn::Dot { dot_token_idx } => Precedence::Dot,
        }
    }
}
