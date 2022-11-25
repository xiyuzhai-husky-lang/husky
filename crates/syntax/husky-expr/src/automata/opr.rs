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
        inputs: Vec<(RangedIdentifier, Option<ExprIdx>)>,
        start: TextPosition,
    },
}

impl OnStackOpr {
    pub(super) fn binary(opr: BinaryOpr) -> Self {
        let precedence = opr.into();
        Self {
            precedence,
            variant: OnStackOprVariant::Binary(opr),
        }
    }

    pub(super) fn prefix(prefix: PrefixOpr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::Prefix,
            variant: OnStackOprVariant::Prefix { prefix, start },
        }
    }

    pub(super) fn list_item(position: TextPosition) -> Self {
        Self {
            precedence: Precedence::None,
            variant: OnStackOprVariant::ListItem(position),
        }
    }

    pub(super) fn list_start(bra: Bracket, attr: ListStartAttr, start: TextPosition) -> Self {
        Self {
            precedence: Precedence::None,
            variant: OnStackOprVariant::ListStart { bra, attr, start },
        }
    }

    pub(super) fn lambda_head(
        inputs: Vec<(RangedIdentifier, Option<ExprIdx>)>,
        start: TextPosition,
    ) -> Self {
        Self {
            precedence: Precedence::LambdaHead,
            variant: OnStackOprVariant::LambdaHead { inputs, start },
        }
    }
}
