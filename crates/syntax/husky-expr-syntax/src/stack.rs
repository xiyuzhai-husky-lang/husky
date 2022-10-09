use crate::*;
use husky_opn_syntax::*;

pub(crate) struct ExprStack<'a> {
    arena: &'a mut RawExprArena,
    oprs: Vec<ExprStackOpr>,
    exprs: Vec<RawExpr>,
    state: ExprStackState,
}

pub struct ExprStackState {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ExprStackOpr {
    precedence: Precedence,
    variant: ExprStackOprVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ExprStackOprVariant {
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
