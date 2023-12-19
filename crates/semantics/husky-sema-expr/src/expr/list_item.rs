use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaCommaListItem {
    pub sema_expr_idx: SemaExprIdx,
    pub comma_regional_token_idx: Option<RegionalTokenIdx>,
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_comma_list_item<E: ExpectFluffyTerm>(
        &mut self,
        syn_comma_list_item: SynCommaListItem,
        expr_ty_expectation: E,
    ) -> SemaCommaListItem {
        let (sema_expr_idx, expectation_outcome) = self
            .build_sema_expr_with_outcome(syn_comma_list_item.syn_expr_idx(), expr_ty_expectation);
        SemaCommaListItem {
            sema_expr_idx,
            comma_regional_token_idx: syn_comma_list_item.comma_regional_token_idx(),
        }
    }

    pub(crate) fn build_sema_comma_list_item_with_its_ty_returned<E: ExpectFluffyTerm>(
        &mut self,
        syn_comma_list_item: SynCommaListItem,
        expr_ty_expectation: E,
    ) -> (SemaCommaListItem, Option<FluffyTerm>) {
        let (sema_expr_idx, ty) =
            self.build_sema_expr_with_ty(syn_comma_list_item.syn_expr_idx(), expr_ty_expectation);
        (
            SemaCommaListItem {
                sema_expr_idx,
                comma_regional_token_idx: syn_comma_list_item.comma_regional_token_idx(),
            },
            ty,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaRegularCallListItem {
    pub argument_expr_idx: SemaExprIdx,
    pub coersion_outcome: Option<ExpectCoersionOutcome>,
    separator: CallListSeparator,
}

impl SemaRegularCallListItem {
    pub(crate) fn new(
        argument_expr_idx: SemaExprIdx,
        coersion_outcome: Option<ExpectCoersionOutcome>,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            argument_expr_idx,
            coersion_outcome,
            separator,
        }
    }

    pub fn argument_sema_expr_idx(&self) -> SemaExprIdx {
        self.argument_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaVariadicCallListItem {
    argument_sema_expr_idx: SemaExprIdx,
    coersion_outcome: Option<ExpectCoersionOutcome>,
    separator: CallListSeparator,
}

impl SemaVariadicCallListItem {
    pub(crate) fn new(
        argument_sema_expr_idx: SemaExprIdx,
        coersion_outcome: Option<ExpectCoersionOutcome>,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            argument_sema_expr_idx,
            coersion_outcome,
            separator,
        }
    }

    pub fn argument_expr_idx(&self) -> SemaExprIdx {
        self.argument_sema_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaKeyedCallListItem {
    key_regional_token_idx: RegionalTokenIdx,
    key: Ident,
    argument_sema_expr_idx: SemaExprIdx,
    coersion_outcome: Option<ExpectCoersionOutcome>,
    separator: CallListSeparator,
}

impl SemaKeyedCallListItem {
    pub(crate) fn new(
        key_regional_token_idx: RegionalTokenIdx,
        key: Ident,
        argument_sema_expr_idx: SemaExprIdx,
        coersion_outcome: Option<ExpectCoersionOutcome>,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            key_regional_token_idx,
            key,
            argument_sema_expr_idx,
            coersion_outcome,
            separator,
        }
    }

    pub fn key_regional_token_idx(&self) -> RegionalTokenIdx {
        self.key_regional_token_idx
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn argument_expr_idx(&self) -> SemaExprIdx {
        self.argument_sema_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_keyed_call_list_item(
        &mut self,
        item: SynKeyedCallListItem,
        param: FluffyRitchieKeyedParameter,
    ) -> SemaKeyedCallListItem {
        let (argument_sema_expr_idx, coersion_outcome) = self.build_sema_expr_with_outcome(
            item.argument_syn_expr_idx(),
            ExpectCoersion::new(param.contract(), param.ty()),
        );
        SemaKeyedCallListItem {
            key_regional_token_idx: item.key_regional_token_idx(),
            key: item.key(),
            argument_sema_expr_idx,
            coersion_outcome,
            separator: item.separator(),
        }
    }
}
