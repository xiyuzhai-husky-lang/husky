use parsec::ParseFromStream;
use smallvec::SmallVec;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum IncompleteExpr {
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
        opr: IncompleteListOpr,
        // todo: move this into opr
        bra: Bracket,
        // todo: move this into opr
        bra_token_idx: TokenIdx,
        // todo: use SmallVec
        items: Vec<Expr>,
        // todo: use SmallVec
        commas: Commas,
    },
    FnCallKeyedArgumentList {
        function: ExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        bra: Bracket,
        lpar_token_idx: TokenIdx,
        // todo: use SmallVec
        arguments: ExprIdxRange,
        // todo: use SmallVec
        keyed_arguments: SmallVec<[KeyedArgumentExpr; 2]>,
        commas: Commas,
    },
    MethodFnCallKeyedArgumentList {
        self_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<ImplicitArgumentList>,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        // todo: use SmallVec
        arguments: ExprIdxRange,
        // todo: use SmallVec
        commas: Commas,
        keyed_arguments: SmallVec<[KeyedArgumentExpr; 2]>,
    },
    LambdaHead {
        // todo: use SmallVec
        inputs: Vec<(RangedIdent, Option<ExprIdx>)>,
        start: TextPosition,
    },
    Application {
        function: Expr,
    },
    /// just needs the return type
    Ritchie {
        ritchie_kind_token_idx: TokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LeftParenthesisToken,
        // todo: use SmallVec
        argument_tys: ExprIdxRange,
        // todo: use SmallVec
        commas: Commas,
        rpar_token_idx: TokenIdx,
        light_arrow_token: LightArrowToken,
    },
    KeyedArgument {
        key_token_idx: TokenIdx,
        key: Ident,
        eq_token: EqToken,
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

impl IncompleteExpr {
    pub(super) fn precedence(&self) -> Precedence {
        match self {
            IncompleteExpr::Binary { punctuation, .. } => (*punctuation).into(),
            IncompleteExpr::Prefix { .. } => Precedence::Prefix,
            IncompleteExpr::ListItem { .. }
            | IncompleteExpr::List { .. }
            | IncompleteExpr::FnCallKeyedArgumentList { .. }
            | IncompleteExpr::MethodFnCallKeyedArgumentList { .. } => Precedence::None,
            IncompleteExpr::LambdaHead { .. } => Precedence::LambdaHead,
            IncompleteExpr::Application { .. } => Precedence::Application,
            IncompleteExpr::Ritchie { .. } => Precedence::Curry,
            IncompleteExpr::KeyedArgument { .. } => Precedence::KeyedArgument,
        }
    }
}
