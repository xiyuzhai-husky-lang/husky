use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::parser) enum IncompleteCommaListOpr {
    UnitOrBracketedOrNewTuple,
    Index {
        owner: SynExprIdx,
    },
    BoxList,
    BoxColonList {
        colon_token_idx: TokenIdx,
    },
    NewLambdaHead,
    FunctionInstantiation {},
    FunctionApplicationOrCall {
        function: SynExprIdx,
    },
    RitchieArguments {
        ritchie_kind_token_idx: TokenIdx,
        ritchie_kind: RitchieKind,
        lpar_token: LparToken,
    },
    TemplateInstantiation {
        template: SynExprIdx,
    },
    MethodInstantiation {
        self_expr: SynExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
    },
    #[deprecated(note = "move this to CallList")]
    MethodApplicationOrCall {
        self_expr: SynExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentToken,
        generic_arguments: Option<SynGenericArgumentList>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ListStartAttr {
    None,
    Attach,
    MethodAttach { ranged_ident: RangedIdent },
}

impl ListStartAttr {
    pub fn attached(&self) -> bool {
        match self {
            ListStartAttr::None => false,
            ListStartAttr::Attach => true,
            ListStartAttr::MethodAttach { .. } => true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListEndAttr {
    None,
    Attach,
    Modulo,
}
