use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SynCommaListItem {
    expr_idx: SynExprIdx,
    comma_token_idx: Option<RegionalTokenIdx>,
}

impl SynCommaListItem {
    pub(crate) fn new(expr_idx: SynExprIdx, comma_token_idx: Option<RegionalTokenIdx>) -> Self {
        Self {
            expr_idx,
            comma_token_idx,
        }
    }

    pub fn expr_idx(self) -> SynExprIdx {
        self.expr_idx
    }

    pub fn comma_token_idx(self) -> Option<RegionalTokenIdx> {
        self.comma_token_idx
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum CallListItem {
    RegularOrVariadic(RegularOrVariadicCallListItem),
    Keyed(KeyedCallListItem),
}

impl From<SynCommaListItem> for CallListItem {
    fn from(item: SynCommaListItem) -> Self {
        CallListItem::RegularOrVariadic(RegularOrVariadicCallListItem {
            argument_expr_idx: item.expr_idx,
            separator: match item.comma_token_idx {
                Some(comma_token_idx) => CallListSeparator::Comma(comma_token_idx),
                None => CallListSeparator::None,
            },
        })
    }
}

impl CallListItem {
    pub fn new_regular(argument_expr_idx: SynExprIdx, comma: Option<RegionalTokenIdx>) -> Self {
        CallListItem::RegularOrVariadic(RegularOrVariadicCallListItem {
            separator: match comma {
                Some(comma_token_idx) => CallListSeparator::Comma(comma_token_idx),
                None => CallListSeparator::None,
            },
            argument_expr_idx,
        })
    }

    pub fn separator(&self) -> CallListSeparator {
        match self {
            CallListItem::RegularOrVariadic(RegularOrVariadicCallListItem {
                separator, ..
            })
            | CallListItem::Keyed(KeyedCallListItem { separator, .. }) => *separator,
        }
    }

    pub(crate) fn set_separator(&mut self, new_separator: CallListSeparator) {
        match self {
            CallListItem::RegularOrVariadic(RegularOrVariadicCallListItem {
                separator, ..
            })
            | CallListItem::Keyed(KeyedCallListItem { separator, .. }) => {
                debug_assert_eq!(*separator, CallListSeparator::None);
                *separator = new_separator
            }
        }
    }

    pub fn argument_expr_idx(&self) -> SynExprIdx {
        match self {
            CallListItem::RegularOrVariadic(RegularOrVariadicCallListItem {
                argument_expr_idx,
                ..
            })
            | CallListItem::Keyed(KeyedCallListItem {
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
pub struct RegularOrVariadicCallListItem {
    argument_expr_idx: SynExprIdx,
    separator: CallListSeparator,
}

impl RegularOrVariadicCallListItem {
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
pub struct KeyedCallListItem {
    key_token_idx: RegionalTokenIdx,
    key: Ident,
    argument_expr_idx: SynExprIdx,
    separator: CallListSeparator,
}

impl KeyedCallListItem {
    pub(crate) fn new(
        key_token_idx: RegionalTokenIdx,
        key: Ident,
        argument_expr_idx: SynExprIdx,
        separator: CallListSeparator,
    ) -> Self {
        Self {
            key_token_idx,
            key,
            argument_expr_idx,
            separator,
        }
    }

    pub fn key_token_idx(&self) -> RegionalTokenIdx {
        self.key_token_idx
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn argument_expr_idx(&self) -> SynExprIdx {
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
