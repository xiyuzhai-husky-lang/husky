use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::parser) enum IncompleteCallListOpr {
    FunctionCall {
        function: SynExprIdx,
        generic_arguments: Option<SynGenericArgumentList>,
    },
    MethodCall {
        self_expr: SynExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        generic_arguments: Option<SynGenericArgumentList>,
    },
}
