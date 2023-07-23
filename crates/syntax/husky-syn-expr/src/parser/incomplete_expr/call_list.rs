use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::parser) enum IncompleteCallListOpr {
    FunctionCall {
        function: SynExprIdx,
        implicit_arguments: Option<SynImplicitArgumentList>,
    },
    MethodCall {
        self_expr: SynExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        implicit_arguments: Option<SynImplicitArgumentList>,
    },
}
