use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaCommaListItem {
    expr_idx: SemaExprIdx,
    comma_regional_token_idx: Option<RegionalTokenIdx>,
}

impl SemaCommaListItem {
    pub(crate) fn new(
        expr_idx: SemaExprIdx,
        comma_regional_token_idx: Option<RegionalTokenIdx>,
    ) -> Self {
        Self {
            expr_idx,
            comma_regional_token_idx,
        }
    }

    pub fn expr_idx(self) -> SemaExprIdx {
        self.expr_idx
    }

    pub fn comma_regional_token_idx(self) -> Option<RegionalTokenIdx> {
        self.comma_regional_token_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum SemaCallListItem {
    RegularOrVariadic(SemaRegularOrVariadicCallListItem),
    Keyed(SemaKeyedCallListItem),
}

impl From<SemaCommaListItem> for SemaCallListItem {
    fn from(item: SemaCommaListItem) -> Self {
        SemaCallListItem::RegularOrVariadic(SemaRegularOrVariadicCallListItem {
            argument_expr_idx: item.expr_idx,
            separator: match item.comma_regional_token_idx {
                Some(comma_regional_token_idx) => {
                    CallListSeparator::Comma(comma_regional_token_idx)
                }
                None => CallListSeparator::None,
            },
        })
    }
}

impl SemaCallListItem {
    pub fn new_regular(argument_expr_idx: SemaExprIdx, comma: Option<RegionalTokenIdx>) -> Self {
        SemaCallListItem::RegularOrVariadic(SemaRegularOrVariadicCallListItem {
            separator: match comma {
                Some(comma_regional_token_idx) => {
                    CallListSeparator::Comma(comma_regional_token_idx)
                }
                None => CallListSeparator::None,
            },
            argument_expr_idx,
        })
    }

    pub fn separator(&self) -> CallListSeparator {
        match self {
            SemaCallListItem::RegularOrVariadic(SemaRegularOrVariadicCallListItem {
                separator,
                ..
            })
            | SemaCallListItem::Keyed(SemaKeyedCallListItem { separator, .. }) => *separator,
        }
    }

    pub(crate) fn set_separator(&mut self, new_separator: CallListSeparator) {
        match self {
            SemaCallListItem::RegularOrVariadic(SemaRegularOrVariadicCallListItem {
                separator,
                ..
            })
            | SemaCallListItem::Keyed(SemaKeyedCallListItem { separator, .. }) => {
                debug_assert_eq!(*separator, CallListSeparator::None);
                *separator = new_separator
            }
        }
    }

    pub fn argument_expr_idx(&self) -> SemaExprIdx {
        match self {
            SemaCallListItem::RegularOrVariadic(SemaRegularOrVariadicCallListItem {
                argument_expr_idx,
                ..
            })
            | SemaCallListItem::Keyed(SemaKeyedCallListItem {
                argument_expr_idx, ..
            }) => *argument_expr_idx,
        }
    }
}

#[test]
fn call_list_item_field_alignment() {
    // todo
    //     let a =
    // CallListItem::RegularOrVariadic(RegularOrVariadicCallListItem {
    //     separator, ..
    // })
    // | CallListItem::Keyed(SemaKeyedCallListItem { separator, .. }) => {
    //     debug_assert_eq!(*separator, CallListSeparator::None);
    //     *separator = new_separator
    // }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaRegularOrVariadicCallListItem {
    argument_expr_idx: SemaExprIdx,
    separator: CallListSeparator,
}

impl SemaRegularOrVariadicCallListItem {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CallListSeparator {
    None,
    Comma(RegionalTokenIdx),
    Semicolon(RegionalTokenIdx),
}
