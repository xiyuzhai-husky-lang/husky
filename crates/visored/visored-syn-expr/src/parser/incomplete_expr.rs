mod call_list;
mod separated_list;

pub(super) use self::call_list::*;
pub(super) use self::separated_list::*;

use super::*;
use crate::expr::VdSynExprData;
use expr::list_item::VdSynSeparatedListItem;
use visored_opr::{delimiter::VdDelimiter, precedence::VdPrecedence};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum IncompleteVdSynExprData {
    Binary {
        lopd: VdSynExprData,
        opr: VdSynExprData,
    },
    Prefix {
        opr: VdSynExprData,
    },
    /// list separated by commas
    /// ```husky
    /// A(a, b, c)
    /// ```
    SeparatedList {
        opr: IncompleteSeparatedListOpr,
        bra: VdDelimiter,
        items: SmallVec<[VdSynSeparatedListItem; 4]>,
    },
    /// call list includes more separators like `;`
    CallList {
        opr: IncompleteCallListOpr,
        items: SmallVec<[VdSynSeparatedListItem; 4]>,
    },
}

impl IncompleteVdSynExprData {
    pub(super) fn precedence(&self) -> VdPrecedence {
        match self {
            IncompleteVdSynExprData::Binary { opr, .. } => todo!(),
            IncompleteVdSynExprData::Prefix { opr, .. } => todo!(),
            IncompleteVdSynExprData::SeparatedList { .. }
            | IncompleteVdSynExprData::CallList { .. } => todo!(),
        }
    }
}
