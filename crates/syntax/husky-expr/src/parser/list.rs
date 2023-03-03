use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum UnfinishedListOpr {
    NewTuple,
    Index {
        owner: ExprIdx,
    },
    BoxList,
    BoxColonList {
        colon_token_idx: TokenIdx,
    },
    NewLambdaHead,
    FunctionInstantiation {},
    FunctionCall {
        function: ExprIdx,
    },
    TemplateInstantiation {
        template: ExprIdx,
    },
    MethodInstantiation {
        self_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
    },
    MethodCall {
        self_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
        implicit_arguments: Option<ImplicitArgumentList>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ListStartAttr {
    None,
    Attach,
    MethodAttach { ranged_ident: RangedIdentifier },
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
