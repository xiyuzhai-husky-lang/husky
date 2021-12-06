//! Entry point for call-hierarchy

use common::*;

use hir::Semantics;
use ide_db::{
    defs::{Definition, NameClass, NameRefClass},
    helpers::pick_best_token,
    search::FileReference,
    FxIndexMap, IdeDatabase,
};
use syntax::ast;

use crate::{goto_definition, FilePosition, NavigationTarget, RangeInfo, TryToNav};

#[derive(Debug, Clone)]
pub struct CallItem {
    pub target: NavigationTarget,
    pub ranges: Vec<TextRange>,
}

pub(crate) fn call_hierarchy(
    db: &IdeDatabase,
    position: FilePosition,
) -> Option<RangeInfo<Vec<NavigationTarget>>> {
    goto_definition::goto_definition(db, position)
}

pub(crate) fn incoming_calls(
    db: &IdeDatabase,
    FilePosition { file_id, offset }: FilePosition,
) -> Option<Vec<CallItem>> {
    todo!()
}

pub(crate) fn outgoing_calls(db: &IdeDatabase, position: FilePosition) -> Option<Vec<CallItem>> {
    todo!()
}

#[derive(Default)]
struct CallLocations {
    funcs: FxIndexMap<NavigationTarget, Vec<TextRange>>,
}

impl CallLocations {
    fn add(&mut self, target: NavigationTarget, range: TextRange) {
        self.funcs.entry(target).or_default().push(range);
    }

    fn into_items(self) -> Vec<CallItem> {
        self.funcs
            .into_iter()
            .map(|(target, ranges)| CallItem { target, ranges })
            .collect()
    }
}
