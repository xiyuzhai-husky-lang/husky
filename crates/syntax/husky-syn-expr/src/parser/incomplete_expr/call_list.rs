use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::parser) enum IncompleteCallListOpr {
    FunctionCall {
        function: SynExprIdx,
        generic_arguments: Option<SynGenericArgumentList>,
    },
    MethodCall {
        self_expr: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        generic_arguments: Option<SynGenericArgumentList>,
    },
}
