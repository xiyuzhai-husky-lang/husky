use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct OnStackOpr {
    precedence: Precedence,
    pub(super) variant: OnStackOprVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum OnStackOprVariant {
    Binary(BinaryOpr),
    ListItem(TextPosition),
    Prefix {
        prefix: PrefixOpr,
        start: TextPosition,
    },
    ListStart {
        bra: Bracket,
        attr: ListStartAttr,
        start: TextPosition,
    },
    LambdaHead {
        inputs: Vec<(RangedCustomIdentifier, Option<RawExprIdx>)>,
        start: TextPosition,
    },
}
