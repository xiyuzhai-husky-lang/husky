use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaCommaListItem {
    sema_expr_idx: SemaExprIdx,
    comma_regional_token_idx: Option<RegionalTokenIdx>,
}

impl<'a> SemaExprEngine<'a> {
    pub(crate) fn build_sema_comma_list_item<E: ExpectFluffyTerm>(
        &mut self,
        syn_comma_list_item: SynCommaListItem,
        expr_ty_expectation: E,
    ) -> SemaCommaListItem {
        SemaCommaListItem {
            sema_expr_idx: self
                .build_sema_expr(syn_comma_list_item.syn_expr_idx(), expr_ty_expectation),
            comma_regional_token_idx: syn_comma_list_item.comma_regional_token_idx(),
        }
    }

    pub(crate) fn build_sema_comma_list_item_with_its_ty_returned<E: ExpectFluffyTerm>(
        &mut self,
        syn_comma_list_item: SynCommaListItem,
        expr_ty_expectation: E,
    ) -> (SemaCommaListItem, Option<FluffyTerm>) {
        let (sema_expr_idx, ty) = self.build_sema_expr_with_its_ty_returned(
            syn_comma_list_item.syn_expr_idx(),
            expr_ty_expectation,
        );
        (
            SemaCommaListItem {
                sema_expr_idx,
                comma_regional_token_idx: syn_comma_list_item.comma_regional_token_idx(),
            },
            ty,
        )
    }
}

impl SemaCommaListItem {
    pub fn sema_expr_idx(self) -> SemaExprIdx {
        self.sema_expr_idx
    }

    pub fn comma_regional_token_idx(self) -> Option<RegionalTokenIdx> {
        self.comma_regional_token_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaRegularCallListItem {
    argument_expr_idx: SemaExprIdx,
    separator: CallListSeparator,
}

impl SemaRegularCallListItem {
    pub(crate) fn new(argument_expr_idx: SemaExprIdx, separator: CallListSeparator) -> Self {
        Self {
            argument_expr_idx,
            separator,
        }
    }

    pub fn argument_expr_idx(&self) -> SemaExprIdx {
        self.argument_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaVariadicCallListItem {
    argument_sema_expr_idx: SemaExprIdx,
    separator: CallListSeparator,
}

impl SemaVariadicCallListItem {
    pub(crate) fn new(argument_sema_expr_idx: SemaExprIdx, separator: CallListSeparator) -> Self {
        Self {
            argument_sema_expr_idx,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaKeyedCallListItem {
    key_regional_token_idx: RegionalTokenIdx,
    key: Ident,
    argument_expr_idx: SemaExprIdx,
    separator: CallListSeparator,
}

impl SemaKeyedCallListItem {
    pub(crate) fn new(
        key_regional_token_idx: RegionalTokenIdx,
        key: Ident,
        argument_expr_idx: SemaExprIdx,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            key_regional_token_idx,
            key,
            argument_expr_idx,
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
        self.argument_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}
