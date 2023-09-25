use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::parser) enum IncompleteCommaListOpr {
    UnitOrBracketedOrNewTuple,
    Index {
        owner: SynExprIdx,
    },
    BoxList,
    BoxColonList {
        colon_regional_token_idx: RegionalTokenIdx,
    },
    NewLambdaHead,
    FunctionInstantiation {},
    FunctionApplicationOrCall {
        function: SynExprIdx,
    },
    RitchieArguments {
        ritchie_kind_regional_token_idx: RegionalTokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LparRegionalToken,
    },
    TemplateInstantiation {
        template: SynExprIdx,
    },
    MethodInstantiation {
        self_expr: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
    },
    #[deprecated(note = "move this to CallList")]
    MethodApplicationOrCall {
        self_expr: SynExprIdx,
        dot_regional_token_idx: RegionalTokenIdx,
        ident_token: IdentRegionalToken,
        generic_arguments: Option<SynTemplateArgumentList>,
    },
}
