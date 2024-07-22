use super::*;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaCommaListItem {
    pub sem_expr_idx: SemExprIdx,
    pub comma_regional_token_idx: Option<RegionalTokenIdx>,
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_sem_comma_list_item<E: ExpectFlyTerm>(
        &mut self,
        syn_comma_list_item: SynCommaListItem,
        expr_ty_expectation: E,
    ) -> SemaCommaListItem {
        let (sem_expr_idx, expectation_outcome) =
            self.build_expr_with_outcome(syn_comma_list_item.syn_expr_idx(), expr_ty_expectation);
        SemaCommaListItem {
            sem_expr_idx,
            comma_regional_token_idx: syn_comma_list_item.comma_regional_token_idx(),
        }
    }

    pub(crate) fn build_sem_comma_list_item_with_its_ty_returned<E: ExpectFlyTerm>(
        &mut self,
        syn_comma_list_item: SynCommaListItem,
        expr_ty_expectation: E,
    ) -> (SemaCommaListItem, Option<FlyTerm>) {
        let (sem_expr_idx, ty) =
            self.build_expr_with_ty(syn_comma_list_item.syn_expr_idx(), expr_ty_expectation);
        (
            SemaCommaListItem {
                sem_expr_idx,
                comma_regional_token_idx: syn_comma_list_item.comma_regional_token_idx(),
            },
            ty,
        )
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaSimpleArgument {
    pub argument_expr_idx: SemExprIdx,
    pub coercion_outcome: Option<ExpectCoercionOutcome>,
    separator: CallListSeparator,
}

/// # constructor
impl SemaSimpleArgument {
    pub(crate) fn new(
        argument_expr_idx: SemExprIdx,
        coercion_outcome: Option<ExpectCoercionOutcome>,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            argument_expr_idx,
            coercion_outcome,
            separator,
        }
    }
}

/// # getters
impl SemaSimpleArgument {
    pub fn argument_sem_expr_idx(&self) -> SemExprIdx {
        self.argument_expr_idx
    }

    pub fn coercion_outcome(&self) -> Option<&ExpectCoercionOutcome> {
        self.coercion_outcome.as_ref()
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaVariadicCallListItem {
    argument_sem_expr_idx: SemExprIdx,
    coercion_outcome: Option<ExpectCoercionOutcome>,
    separator: CallListSeparator,
}

impl SemaVariadicCallListItem {
    pub(crate) fn new(
        argument_sem_expr_idx: SemExprIdx,
        coercion_outcome: Option<ExpectCoercionOutcome>,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            argument_sem_expr_idx,
            coercion_outcome,
            separator,
        }
    }

    pub fn argument_expr_idx(&self) -> SemExprIdx {
        self.argument_sem_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemaKeyedCallListItem {
    key_regional_token_idx: RegionalTokenIdx,
    key: Ident,
    argument_sem_expr_idx: SemExprIdx,
    coercion_outcome: Option<ExpectCoercionOutcome>,
    separator: CallListSeparator,
}

impl SemaKeyedCallListItem {
    pub(crate) fn new(
        key_regional_token_idx: RegionalTokenIdx,
        key: Ident,
        argument_sem_expr_idx: SemExprIdx,
        coercion_outcome: Option<ExpectCoercionOutcome>,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            key_regional_token_idx,
            key,
            argument_sem_expr_idx,
            coercion_outcome,
            separator,
        }
    }

    pub fn key_regional_token_idx(&self) -> RegionalTokenIdx {
        self.key_regional_token_idx
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn argument_expr_idx(&self) -> SemExprIdx {
        self.argument_sem_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_sem_keyed_call_list_item(
        &mut self,
        item: SynKeyedCallListItem,
        param: FlyRitchieKeyedParameter,
    ) -> SemaKeyedCallListItem {
        let (argument_sem_expr_idx, coercion_outcome) = self.build_expr_with_outcome(
            item.argument_syn_expr_idx(),
            ExpectCoercion::new(param.contract(), param.ty()),
        );
        SemaKeyedCallListItem {
            key_regional_token_idx: item.key_regional_token_idx(),
            key: item.key(),
            argument_sem_expr_idx,
            coercion_outcome,
            separator: item.separator(),
        }
    }
}
