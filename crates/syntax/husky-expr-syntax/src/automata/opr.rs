use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) struct OnStackOpr {
    precedence: Precedence,
    pub(super) variant: OnStackOprVariant,
}

impl OnStackOpr {
    pub(super) fn precedence(&self) -> Precedence {
        self.precedence
    }
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
