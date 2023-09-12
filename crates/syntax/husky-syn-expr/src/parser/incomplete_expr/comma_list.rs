use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::parser) enum IncompleteCommaListOpr {
    UnitOrBracketedOrNewTuple,
    Index {
        owner: SynExprIdx,
    },
    BoxList,
    BoxColonList {
        colon_token_idx: RegionalTokenIdx,
    },
    NewLambdaHead,
    FunctionInstantiation {},
    FunctionApplicationOrCall {
        function: SynExprIdx,
    },
    RitchieArguments {
        ritchie_kind_token_idx: RegionalTokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: RegionalLParToken,
    },
    TemplateInstantiation {
        template: SynExprIdx,
    },
    MethodInstantiation {
        self_expr: SynExprIdx,
        dot_token_idx: RegionalTokenIdx,
        ident_token: RegionalIdentToken,
    },
    #[deprecated(note = "move this to CallList")]
    MethodApplicationOrCall {
        self_expr: SynExprIdx,
        dot_token_idx: RegionalTokenIdx,
        ident_token: RegionalIdentToken,
        generic_arguments: Option<SynGenericArgumentList>,
    },
}
