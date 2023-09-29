use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynCommaListItem {
    syn_expr_idx: SynExprIdx,
    comma_regional_token_idx: Option<RegionalTokenIdx>,
}

impl SynCommaListItem {
    pub(crate) fn new(
        syn_expr_idx: SynExprIdx,
        comma_regional_token_idx: Option<RegionalTokenIdx>,
    ) -> Self {
        Self {
            syn_expr_idx,
            comma_regional_token_idx,
        }
    }

    pub fn syn_expr_idx(self) -> SynExprIdx {
        self.syn_expr_idx
    }

    pub fn comma_regional_token_idx(self) -> Option<RegionalTokenIdx> {
        self.comma_regional_token_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum SynCallListItem {
    RegularOrVariadic(SynRegularOrVariadicCallListItem),
    Keyed(SynKeyedCallListItem),
}

impl From<SynCommaListItem> for SynCallListItem {
    fn from(item: SynCommaListItem) -> Self {
        SynCallListItem::RegularOrVariadic(SynRegularOrVariadicCallListItem {
            argument_expr_idx: item.syn_expr_idx,
            separator: match item.comma_regional_token_idx {
                Some(comma_regional_token_idx) => {
                    CallListSeparator::Comma(comma_regional_token_idx)
                }
                None => CallListSeparator::None,
            },
        })
    }
}

impl SynCallListItem {
    pub fn new_regular(argument_expr_idx: SynExprIdx, comma: Option<RegionalTokenIdx>) -> Self {
        SynCallListItem::RegularOrVariadic(SynRegularOrVariadicCallListItem {
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
            SynCallListItem::RegularOrVariadic(SynRegularOrVariadicCallListItem {
                separator,
                ..
            })
            | SynCallListItem::Keyed(SynKeyedCallListItem { separator, .. }) => *separator,
        }
    }

    pub(crate) fn set_separator(&mut self, new_separator: CallListSeparator) {
        match self {
            SynCallListItem::RegularOrVariadic(SynRegularOrVariadicCallListItem {
                separator,
                ..
            })
            | SynCallListItem::Keyed(SynKeyedCallListItem { separator, .. }) => {
                debug_assert_eq!(*separator, CallListSeparator::None);
                *separator = new_separator
            }
        }
    }

    pub fn argument_expr_idx(&self) -> SynExprIdx {
        match self {
            SynCallListItem::RegularOrVariadic(SynRegularOrVariadicCallListItem {
                argument_expr_idx,
                ..
            })
            | SynCallListItem::Keyed(SynKeyedCallListItem {
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
    // | CallListItem::Keyed(KeyedCallListItem { separator, .. }) => {
    //     debug_assert_eq!(*separator, CallListSeparator::None);
    //     *separator = new_separator
    // }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynRegularOrVariadicCallListItem {
    argument_expr_idx: SynExprIdx,
    separator: CallListSeparator,
}

impl SynRegularOrVariadicCallListItem {
    pub(crate) fn new(argument_expr_idx: SynExprIdx, separator: CallListSeparator) -> Self {
        Self {
            argument_expr_idx,
            separator,
        }
    }

    pub fn argument_expr_idx(&self) -> SynExprIdx {
        self.argument_expr_idx
    }

    pub fn separator(&self) -> CallListSeparator {
        self.separator
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynKeyedCallListItem {
    key_regional_token_idx: RegionalTokenIdx,
    key: Ident,
    argument_expr_idx: SynExprIdx,
    separator: CallListSeparator,
}

impl SynKeyedCallListItem {
    pub(crate) fn new(
        key_regional_token_idx: RegionalTokenIdx,
        key: Ident,
        argument_expr_idx: SynExprIdx,
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

    pub fn argument_syn_expr_idx(&self) -> SynExprIdx {
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
